string[] names = ["a", "b", "c"];

// Task.Run runs the work on a pool thread. It can return a value.
var worker = Task.Run(() =>
{
    foreach (var n in names)
    {
        Console.WriteLine($"worker sees {n}");
    }
    return names.Length;
});

var count = await worker;    // wait for it
Console.WriteLine($"worker counted {count}");
