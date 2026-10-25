using System.Threading.Channels;

var ch = Channel.CreateUnbounded<List<int>>();

var receiver = Task.Run(async () =>
{
    var got = await ch.Reader.ReadAsync();
    await Task.Delay(50);   // a little work before looking at it
    Console.WriteLine($"receiver sees [{string.Join(", ", got)}]");
});

List<int> msg = [1, 2];
await ch.Writer.WriteAsync(msg);
msg[0] = 99;   // sender keeps changing what it already sent
await receiver;
