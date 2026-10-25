string? name = Find(2);

// The ! says "trust me, not null". The compiler stays quiet, and nothing is checked here.
string shown = name!;
Console.WriteLine("assigned without a crash");

// The null surfaces later, away from the ! that hid it.
Console.WriteLine(shown.Length);

static string? Find(int id) => id == 1 ? "ana" : null;
