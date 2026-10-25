var go = new ManualResetEventSlim(false);
var never = new ManualResetEventSlim(false);

var foreground = new Thread(() =>
{
    go.Wait();
    Console.WriteLine("foreground: finished after Main returned");
});
foreground.Start(); // foreground by default

var background = new Thread(() => never.Wait()) { IsBackground = true };
background.Start();

Console.WriteLine("main: returning");
go.Set();
