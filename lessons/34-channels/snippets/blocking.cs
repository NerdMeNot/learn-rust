using System.Collections.Concurrent;
using System.Threading.Channels;

// BlockingCollection blocks a thread like mpsc, and it can be closed
var bc = new BlockingCollection<int>();
bc.Add(1); bc.Add(2); bc.Add(3);
bc.CompleteAdding();   // the writer side closes
foreach (var m in bc.GetConsumingEnumerable()) Console.WriteLine($"BlockingCollection: {m}");   // for m in rx
try { bc.Add(4); } catch (InvalidOperationException) { Console.WriteLine("Add after CompleteAdding: InvalidOperationException"); }

var bounded = new BlockingCollection<int>(2);   // like sync_channel(2): TryAdd is try_send
Console.WriteLine($"TryAdd x3: {bounded.TryAdd(1)}, {bounded.TryAdd(2)}, {bounded.TryAdd(3)}");

// Several consumers on one Channel: each item is read exactly once
var ch = Channel.CreateUnbounded<int>();
for (var i = 0; i < 9; i++) ch.Writer.TryWrite(i);
ch.Writer.Complete();
var taken = new ConcurrentBag<int>();
var readers = Enumerable.Range(0, 3).Select(_ => Task.Run(async () =>
{
    await foreach (var item in ch.Reader.ReadAllAsync()) taken.Add(item);
})).ToArray();
await Task.WhenAll(readers);
Console.WriteLine($"3 readers took [{string.Join(", ", taken.Order())}]");
