using System.Collections.Concurrent;

var gate = new object();
var locked = 0;
var interlocked = 0;
var dict = new ConcurrentDictionary<string, int>();

var threads = Enumerable.Range(0, 8).Select(_ => new Thread(() =>
{
    for (var i = 0; i < 1000; i++)
    {
        lock (gate)
        {
            locked++;
        }
        Interlocked.Increment(ref interlocked);
        dict.AddOrUpdate("hits", 1, (_, old) => old + 1);
    }
})).ToList();
threads.ForEach(t => t.Start());
threads.ForEach(t => t.Join());
Console.WriteLine($"lock: {locked}");
Console.WriteLine($"Interlocked.Increment: {interlocked}");
Console.WriteLine($"ConcurrentDictionary.AddOrUpdate: {dict["hits"]}");
