var path = Path.Combine(Path.GetTempPath(), "rbc-hello.txt");
File.WriteAllText(path, "hello");

using (var f = File.OpenText(path))
{
    var text = f.ReadToEnd();
    Console.WriteLine(text);
}
File.Delete(path);
