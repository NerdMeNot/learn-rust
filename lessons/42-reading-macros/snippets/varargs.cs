static int Sum(params int[] xs) => xs.Sum();

Console.WriteLine($"Sum() = {Sum()}");
Console.WriteLine($"Sum(1, 2, 3, 4) = {Sum(1, 2, 3, 4)}");
