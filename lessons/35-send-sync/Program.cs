string[] words = ["alpha", "beta"];

// Task.Run runs the work on a pool thread; the result comes back through the Task
var back = await Task.Run(() => words);
Console.WriteLine("moved there and back: [" + string.Join(", ", back.Select(w => $"\"{w}\"")) + "]");

// two pool threads read the same array at once; nothing checks that this is safe
var a = Task.Run(() => words[0].Length);
var b = Task.Run(() => words[1].Length);
Console.WriteLine($"lengths read in parallel: {await a} and {await b}");
