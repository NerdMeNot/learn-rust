using System;
using System.Threading.Tasks;

static async Task TaskBody()
{
    Console.WriteLine("task body runs");
    await Task.CompletedTask;
}

Console.WriteLine("start");
Task task = TaskBody(); // runs on this thread until its first await that has to wait
Console.WriteLine("task created");
await task;
Console.WriteLine("end");
