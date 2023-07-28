use sqlx::{SqliteConnection, Connection, Row};
use chess::{Board, Piece, Color, Square};
use indicatif::{ProgressBar, ProgressStyle};
use std::str::FromStr;
use std::mem;
use futures::stream::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    println!("The current directory is {}", current_dir.display());

    println!("Opening old db");
    let mut old_db = SqliteConnection::connect("../../datasets/2021-07-31-lichess-evaluations-37MM.db/test.db").await?;
    println!("Opening new db");
    let mut new_db = SqliteConnection::connect("new.db?mode=rwc").await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS Evaluations (
                id INTEGER PRIMARY KEY,
                fen TEXT NOT NULL,
                input_tensor BLOB NOT NULL,
                eval_for_white REAL NOT NULL)")
        .execute(&mut new_db).await?;

    let count: i64 = sqlx::query("SELECT COUNT(*) as count FROM Evaluations")
        .fetch_one(&mut old_db).await?
        .get("count");

    let progress = ProgressBar::new(count as u64);
    progress.set_style(ProgressStyle::default_bar().template("{wide_bar} {pos}/{len} {elapsed_precise}")?);

    let mut old_data = sqlx::query("SELECT id, fen, eval FROM Evaluations")
        .fetch(&mut old_db);

    let mut tx = new_db.begin().await?;

    while let Some(row) = old_data.try_next().await? {
        let id: i64 = row.get("id");
        let fen: String = row.get("fen");
        let eval: f32 = row.get("eval");
        
        let inputs: Vec<u8> = unsafe { mem::transmute(fen_as_input(&fen)) };
        let normalized_eval = normalize_eval(eval);

        sqlx::query("INSERT INTO Evaluations (id, fen, input_tensor, eval_for_white) VALUES (?1, ?2, ?3, ?4)")
            .bind(id)
            .bind(fen)
            .bind(inputs)
            .bind(normalized_eval)
            .execute(&mut *tx).await?;

        progress.inc(1);
    }
    
    tx.commit().await?;
    progress.finish();

    Ok(())
}

fn fen_as_input(fen: &str) -> Vec<i8> {
    let board = Board::from_str(fen).unwrap();
    let mut inputs: Vec<i8> = vec![0; 65];
    for i in 0..64 {
        let square = unsafe { Square::new(i as u8) };
        if let Some(piece) = board.piece_on(square) {
            inputs[i as usize] = piece_value(piece, board.color_on(square).unwrap());
        }
    }
    inputs[64] = if board.side_to_move() == Color::White { 1 } else { -1 };
    inputs
}

fn piece_value(piece: Piece, color: Color) -> i8 {
    let val = match piece {
        Piece::Pawn => 1,
        Piece::Knight => 2,
        Piece::Bishop => 3,
        Piece::Rook => 4,
        Piece::Queen => 5,
        Piece::King => 6,
    };
    if color == Color::White { val } else { -val }
}

fn normalize_eval(eval: f32) -> f32 {
    eval.max(-15.0).min(15.0) / 10.0
}
