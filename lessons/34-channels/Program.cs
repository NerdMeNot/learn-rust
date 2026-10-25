using System.Threading.Channels;

var ch = Channel.CreateUnbounded<string>();

_ = Task.Run(async () =>
{
    for (var i = 1; i <= 3; i++)
    {
        await ch.Writer.WriteAsync($"job {i}");
    }
    ch.Writer.Complete();            // close the channel
});

await foreach (var msg in ch.Reader.ReadAllAsync())   // ends when it closes
{
    Console.WriteLine($"got {msg}");
}
