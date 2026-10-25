using System;
using System.Threading.Tasks;

// The method waits on a task that we complete later, so the first await really has to wait.
static async Task<string> FetchName(Task gate)
{
    Console.WriteLine("  FetchName body runs");
    await gate;
    return "ferris";
}

var gate = new TaskCompletionSource();
Console.WriteLine("calling FetchName()");
Task<string> task = FetchName(gate.Task);
Console.WriteLine($"called it: the body has already started, completed = {task.IsCompleted}");
gate.SetResult();
Console.WriteLine($"name: {await task}");

Func<Task<string>> later = () => FetchName(Task.CompletedTask);
Console.WriteLine("delegate built; nothing has run for it");
Console.WriteLine($"name: {await later()}");
