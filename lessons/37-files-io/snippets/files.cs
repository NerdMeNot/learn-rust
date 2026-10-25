using System.Text;

try
{
    File.ReadAllText("/no/such/file.txt");
}
catch (IOException e)
{
    Console.WriteLine($"{e.GetType().Name}: {e.Message}");
}

var path = Path.Combine(Path.GetTempPath(), "lr-fio-cs.txt");
File.WriteAllBytes(path, new byte[] { (byte)'b', 0xff, (byte)'x', 10 });
var text = File.ReadAllText(path);
Console.WriteLine("ReadAllText on invalid UTF-8: no exception, chars: " +
    string.Join(" ", text.TrimEnd().Select(c => ((int)c).ToString("x"))));

using (var reader = new StreamReader(path))
{
    Console.WriteLine("first line has " + reader.ReadLine()!.Length + " chars");
} // Dispose runs here

// a FileStream is buffered by default (4 KiB); bufferSize 0 asks for a Rust-style bare File
long WriteCalls() => long.Parse(File.ReadAllLines("/proc/self/io").First(l => l.StartsWith("syscw:"))[6..].Trim());
long ThousandLines(int bufferSize)
{
    using var fs = new FileStream(path, FileMode.Create, FileAccess.Write, FileShare.None, bufferSize);
    var before = WriteCalls();
    for (int i = 0; i < 1000; i++) fs.Write(Encoding.UTF8.GetBytes($"line {i}\n"));
    return WriteCalls() - before;
}
var buffered = ThousandLines(4096);
var bare = ThousandLines(0);
Console.WriteLine($"FileStream default: {buffered} write calls before Dispose, bufferSize 0: {bare}");
File.Delete(path);

// Dispose flushes the buffer, and using reports a failure there.
// /dev/full refuses every byte, like a full disk (Linux).
try
{
    using var w = new StreamWriter("/dev/full");
    w.Write("data");
}
catch (IOException e)
{
    Console.WriteLine("Dispose reported: " + e.Message);
}
Console.WriteLine("Combine(absolute): " + Path.Combine("/data", "/etc/hosts"));
