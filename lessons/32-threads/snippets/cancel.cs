using var source = new CancellationTokenSource();
var worker = new Thread(() =>
{
    source.Token.WaitHandle.WaitOne(); // a real worker checks IsCancellationRequested
    Console.WriteLine("worker: cancellation was requested");
});
worker.Start();
source.Cancel();
worker.Join();
