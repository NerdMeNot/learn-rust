var text = "the cat saw the dog and the cat ran";

// Dictionary: count words. Iteration order isn't promised, so sort before printing.
var counts = new Dictionary<string, int>();
foreach (var w in text.Split(' '))
{
    counts[w] = counts.GetValueOrDefault(w) + 1;
}
var top = counts
    .OrderByDescending(p => p.Value)
    .ThenBy(p => p.Key, StringComparer.Ordinal)
    .Take(3);
Console.WriteLine($"top: {string.Join(", ", top)}");

// SortedDictionary: keys always come out sorted
var byName = new SortedDictionary<string, int>(StringComparer.Ordinal)
{
    ["zoe"] = 31,
    ["ana"] = 30,
    ["mia"] = 28,
};
Console.WriteLine($"sorted: {string.Join(", ", byName)}");

// HashSet: unique items, fast "is it in there?"
var seen = new HashSet<string>(text.Split(' '));
Console.WriteLine($"{seen.Count} unique words, has dog: {seen.Contains("dog")}");

// Queue: add at the back, take from the front
var queue = new Queue<string>(["a", "b"]);
queue.Enqueue("c");
var first = queue.Dequeue();
Console.WriteLine($"served {first}, waiting [{string.Join(", ", queue)}]");

// PriorityQueue gives the smallest priority first
var heap = new PriorityQueue<int, int>();
foreach (var n in new[] { 5, 1, 8, 3 })
{
    heap.Enqueue(n, n);
}
Console.Write("smallest first:");
while (heap.TryDequeue(out var n, out _))
{
    Console.Write($" {n}");
}
Console.WriteLine();
