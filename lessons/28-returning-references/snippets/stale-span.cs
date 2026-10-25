using System.Runtime.InteropServices;

var scores = new List<int>(3) { 3, 9, 4 };
Span<int> top = FirstTwo(scores);
scores.Add(7); // the list is full, so it moves to a bigger array
top[0] = 100;  // writes into the old array
Console.WriteLine($"scores[0]: {scores[0]}");

// returns a piece of the input: a view of it, not a copy
static Span<int> FirstTwo(List<int> xs) => CollectionsMarshal.AsSpan(xs)[..2];
