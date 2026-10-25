var results = new List<int>(10_000_000);   // room for everything up front
var start = new ManualResetEventSlim();
var threads = new List<Thread>();
for (var i = 0; i < 2; i++)
{
    var t = new Thread(() =>
    {
        start.Wait();         // both begin together
        for (var j = 0; j < 5_000_000; j++)
        {
            results.Add(j);   // no lock!
        }
    });
    t.Start();
    threads.Add(t);
}
start.Set();
foreach (var t in threads)
{
    t.Join();
}
Console.WriteLine($"expected 10000000, got {results.Count}");
