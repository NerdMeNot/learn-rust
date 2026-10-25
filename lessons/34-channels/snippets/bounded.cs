using System.Threading.Channels;

// like sync_channel(2): TryWrite is try_send
var ch = Channel.CreateBounded<int>(2);
Console.WriteLine($"TryWrite 1: {ch.Writer.TryWrite(1)}");
Console.WriteLine($"TryWrite 2: {ch.Writer.TryWrite(2)}");
Console.WriteLine($"TryWrite 3: {ch.Writer.TryWrite(3)}");

ch.Writer.Complete();   // the writer side closes, as Go's close does
Console.WriteLine($"TryWrite after Complete: {ch.Writer.TryWrite(4)}");
try
{
    await ch.Writer.WriteAsync(5);
}
catch (ChannelClosedException)
{
    Console.WriteLine("WriteAsync after Complete: ChannelClosedException");
}

// the queued items can still be read, then the reader reports the end
while (await ch.Reader.WaitToReadAsync())
{
    Console.WriteLine($"read: {await ch.Reader.ReadAsync()}");
}
Console.WriteLine("reader finished");

// A bound of 0 is a rendezvous (.NET 10): a write completes only when a reader takes it
var rendezvous = Channel.CreateBounded<int>(0);
Console.WriteLine($"TryWrite on a rendezvous channel with no reader: {rendezvous.Writer.TryWrite(1)}");
var pending = rendezvous.Writer.WriteAsync(2).AsTask();
await Task.Delay(100);
Console.WriteLine($"WriteAsync before any read is done: {pending.IsCompleted}");
Console.WriteLine($"read: {await rendezvous.Reader.ReadAsync()}");
await pending;
Console.WriteLine($"WriteAsync after the read is done: {pending.IsCompleted}");
