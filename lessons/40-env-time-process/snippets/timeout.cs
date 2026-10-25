using System.Diagnostics;

// WaitForExit(milliseconds): false if the child is still running when the time is up
var slow = Process.Start("sleep", "60");
Console.WriteLine($"finished within the limit: {slow.WaitForExit(200)}");
slow.Kill();
slow.WaitForExit();
Console.WriteLine($"ExitCode after Kill: {slow.ExitCode}");

// a child ended by a signal looks like an exit code of 128 + the signal number
var killed = Process.Start("sh", new[] { "-c", "kill -9 $$" });
killed.WaitForExit();
var exited = Process.Start("sh", new[] { "-c", "exit 137" });
exited.WaitForExit();
Console.WriteLine($"kill -9: {killed.ExitCode}, exit 137: {exited.ExitCode}");
