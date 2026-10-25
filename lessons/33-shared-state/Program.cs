var results = new List<int>();
var gate = new object();
var threads = new List<Thread>();

for (var i = 0; i < 3; i++)
{
    var n = i;
    var t = new Thread(() =>
    {
        lock (gate)              // lock, add, unlock
        {
            results.Add(n * 10);
        }
    });
    t.Start();
    threads.Add(t);
}
foreach (var t in threads)
{
    t.Join();
}

results.Sort();
Console.WriteLine($"[{string.Join(", ", results)}]");
