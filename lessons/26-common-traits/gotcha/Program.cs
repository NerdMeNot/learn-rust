var notes = new Dictionary<Version, string>();
notes[new Version(1, 2)] = "stable";
Console.WriteLine($"found: {notes[new Version(1, 2)]}");

record Version(int Major, int Minor);
