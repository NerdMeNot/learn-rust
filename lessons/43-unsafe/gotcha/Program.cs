int[] prices = [10, 20, 30];
fixed (int* p = prices)
{
    int second = *(p + 1);
    Console.WriteLine($"second: {second}");
}
