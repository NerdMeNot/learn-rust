int[] prices = [10, 20, 30];
unsafe
{
    fixed (int* p = prices)
    {
        Console.WriteLine($"second: {*(p + 1)}");
    }
}
