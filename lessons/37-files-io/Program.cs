var path = Path.Combine(Path.GetTempPath(), "rbc-notes.txt");

// write a whole file at once (creates it, or empties it first)
File.WriteAllText(path, "milk\neggs\n");

// open for appending and add a line
File.AppendAllText(path, "bread\n");

// read the whole file into one string
var all = File.ReadAllText(path);
Console.WriteLine($"{all.Length} chars");

// read line by line, without loading the whole file
var i = 1;
foreach (var line in File.ReadLines(path))
{
    Console.WriteLine($"{i++}: {line}");
}

Console.WriteLine($"file name: \"{Path.GetFileName(path)}\"");

// a missing file throws; catch the IOException family
try
{
    File.ReadAllText("/no/such/file.txt");
}
catch (IOException e)
{
    Console.WriteLine($"error: {e.Message} ({e.GetType().Name})");
}

File.Delete(path);
