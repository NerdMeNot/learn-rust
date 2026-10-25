using System.IO;

// C# has no checked exceptions: this compiles with no try and no throws.
Console.WriteLine(File.ReadAllText("missing.txt"));
