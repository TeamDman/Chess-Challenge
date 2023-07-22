using System;
using ChessChallenge.API;

public class MyBot : IChessBot
{
    public Move Think(Board board, Timer timer)
    {
        // store the weights in a long value, with every 8 bits being a separate weight
        var weightsPacked = new long[]{
            9223372036854775807,
            0000000000000000000,
            1111111111111111111,
        };
        // convert to floats
        // var weights = new float[weightsPacked.Length*(64/8)];
        // for (int i=0; i<weightsPacked.Length; i++)
        // {
        //     long weight = weightsPacked[i];
        //     int j = i*(64/8);
        //     weights[j] = (weight & 0xFF) / 255f;
        //     weights[j+1] = ((weight >> 8) & 0xFF) / 255f;
        //     weights[j+2] = ((weight >> 16) & 0xFF) / 255f;
        //     weights[j+3] = ((weight >> 24) & 0xFF) / 255f;
        //     weights[j+4] = ((weight >> 32) & 0xFF) / 255f;
        //     weights[j+5] = ((weight >> 40) & 0xFF) / 255f;
        //     weights[j+6] = ((weight >> 48) & 0xFF) / 255f;
        //     weights[j+7] = ((weight >> 56) & 0xFF) / 255f;
        // }

        // convert to floats
        var weights = new float[weightsPacked.Length * 8];
        for (int i = 0; i < weightsPacked.Length; i++)
        {
            for (int b = 0; b < 8; b++)
            {
                weights[i * 8 + b] = ((weightsPacked[i] >> (b * 8)) & 0xFF) / 255f;
            }
        }
        Console.WriteLine("Weights (" + weights.Length + "): " + string.Join(", ", weights));
        // board.AllPiecesBitboard()
        var moves = board.GetLegalMoves();
        return moves[0];
    }
}