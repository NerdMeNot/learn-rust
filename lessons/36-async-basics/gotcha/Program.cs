using System;
using System.Threading.Tasks;

static async Task<int> LoadCount()
{
    await Task.Delay(1);
    return 3;
}

static async Task<int> Report()
{
    int n = LoadCount();
    return n + 1;
}

Console.WriteLine($"count: {await Report()}");
