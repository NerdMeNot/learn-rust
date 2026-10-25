using System.Diagnostics;

// args doesn't include the program name
Console.WriteLine($"{args.Length} arg(s) after the program name");

// GetEnvironmentVariable returns null when the variable isn't set
var port = Environment.GetEnvironmentVariable("RBC_PORT") ?? "8080";
Console.WriteLine($"port {port}");
var missing = Environment.GetEnvironmentVariable("RBC_MISSING");
Console.WriteLine(missing is not null ? $"RBC_MISSING={missing}" : "RBC_MISSING: not set");

// Stopwatch measures, TimeSpan is an amount of time
var sw = Stopwatch.StartNew();
Thread.Sleep(20);
var took = sw.Elapsed;
Console.WriteLine($"slept at least 20ms: {took >= TimeSpan.FromMilliseconds(20)}");

// run another program and capture what it prints
var info = new ProcessStartInfo("echo") { RedirectStandardOutput = true };
info.ArgumentList.Add("hello from echo");
using var p = Process.Start(info)!;
Console.Write(p.StandardOutput.ReadToEnd());
p.WaitForExit();
Console.WriteLine($"exit ok: {p.ExitCode == 0}");
