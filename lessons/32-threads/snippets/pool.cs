var onPool = await Task.Run(() => Thread.CurrentThread.IsThreadPoolThread);
Console.WriteLine($"Task.Run runs on a pool thread: {onPool}");

bool? dedicated = null;
var t = new Thread(() => dedicated = Thread.CurrentThread.IsThreadPoolThread);
t.Start();
t.Join();
Console.WriteLine($"new Thread runs on a pool thread: {dedicated}");

// a Task that throws keeps the exception until you await it, like Rust's join
var failing = Task.Run(() => { throw new InvalidOperationException("task failed"); });
try
{
    await failing;
}
catch (InvalidOperationException e)
{
    Console.WriteLine($"await rethrew: {e.Message}");
}
