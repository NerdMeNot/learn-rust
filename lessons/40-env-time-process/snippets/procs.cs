using System.Diagnostics;

// args holds only the arguments; Environment.GetCommandLineArgs() includes the program
Console.WriteLine($"args: {args.Length}");

// ArgumentList: each argument is its own string, no shell
var echo = new ProcessStartInfo("echo") { RedirectStandardOutput = true };
foreach (var a in new[] { "a  b", "$HOME", "one; echo two" }) echo.ArgumentList.Add(a);
var p = Process.Start(echo)!;
Console.WriteLine($"output {p.StandardOutput.ReadToEnd().Trim()}");
p.WaitForExit();

// a program that runs and fails: ExitCode, nothing is thrown
var f = Process.Start(new ProcessStartInfo("false"))!;
f.WaitForExit();
Console.WriteLine($"false exits with {f.ExitCode}");

// Stopwatch is the monotonic clock; DateTime.UtcNow is the wall clock
var sw = Stopwatch.StartNew();
Thread.Sleep(20);
Console.WriteLine($"elapsed >= 20ms: {sw.Elapsed >= TimeSpan.FromMilliseconds(20)}");

// TimeSpan is signed
Console.WriteLine($"1s - 2s = {TimeSpan.FromSeconds(1) - TimeSpan.FromSeconds(2)}");

// Environment.SetEnvironmentVariable changes this process; a child started afterwards sees it
Environment.SetEnvironmentVariable("RBC_MODE", "fast");
var env = new ProcessStartInfo("printenv") { RedirectStandardOutput = true };
env.ArgumentList.Add("RBC_MODE");
Console.WriteLine($"child sees {Process.Start(env)!.StandardOutput.ReadToEnd().Trim()}");

try
{
    Console.WriteLine("calling Environment.Exit");
    Environment.Exit(0);
}
finally
{
    Console.WriteLine("finally ran");
}
