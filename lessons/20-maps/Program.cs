var ages = new Dictionary<string, uint>
{
    ["ana"] = 30,
    ["bo"] = 25,
};

if (ages.TryGetValue("ana", out var age))
{
    Console.WriteLine($"ana is {age}");
}
else
{
    Console.WriteLine("no ana");
}

// 0 when the key is missing
var zed = ages.GetValueOrDefault("zed");
Console.WriteLine($"zed is {zed}");

// count words
var counts = new Dictionary<string, uint>();
foreach (var word in "a b a c a".Split(' '))
{
    counts[word] = counts.GetValueOrDefault(word) + 1;
}
Console.WriteLine($"a appears {counts["a"]} times");
