var t = new Thread(() =>
{
    throw new InvalidOperationException("worker failed");
});
t.Start();
t.Join();
Console.WriteLine("main: never reached");
