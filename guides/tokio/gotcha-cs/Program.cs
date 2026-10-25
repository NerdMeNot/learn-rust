using System;
using System.Threading.Tasks;

var gate = new object();
var hits = 0;
await Task.Run(async () =>
{
    lock (gate)
    {
        await Task.Delay(10); // still holding the lock
        hits++;
    }
});
Console.WriteLine(hits);
