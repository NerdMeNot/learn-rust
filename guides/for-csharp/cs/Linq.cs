var evens = Enumerable.Range(1, 6)
    .Where(n => { Console.WriteLine($"check {n}"); return n % 2 == 0; })
    .Select(n => n * 10);
Console.WriteLine("built the query");
Console.WriteLine($"first: {evens.First()}");
Console.WriteLine($"count: {evens.Count()}");   // starts over from 1
