using System;
using System.Threading.Tasks;

static async Task<string> Fetch(string name, int ms)
{
    await Task.Delay(ms);
    return $"{name} done after {ms}ms";
}

// a Task is already running when you get it
var users = Fetch("users", 30);
var orders = Fetch("orders", 10);
Console.WriteLine(await users);
Console.WriteLine(await orders);

var both = await Task.WhenAll(Fetch("left", 20), Fetch("right", 20));
Console.WriteLine($"{both[0]} / {both[1]}");

try
{
    Console.WriteLine(await Fetch("slow", 500).WaitAsync(TimeSpan.FromMilliseconds(50)));
}
catch (TimeoutException)
{
    Console.WriteLine("slow call timed out");
}
