using System;
using System.Collections.Generic;
using System.Linq;

var pq = new PriorityQueue<string, int>();
foreach (var (name, priority) in new[] { ("write", 2), ("deploy", 5), ("test", 1) })
    pq.Enqueue(name, priority);
Console.WriteLine($"Dequeue: {pq.Dequeue()} (the lowest priority number comes out first)");

var log = new SortedDictionary<int, string> { [25] = "login", [10] = "boot", [40] = "upload" };
Console.WriteLine($"SortedDictionary keys: {string.Join(", ", log.Keys)}");

var set = new SortedSet<int> { 5, 1, 9, 3, 7 };
var view = set.GetViewBetween(3, 7);
Console.WriteLine($"GetViewBetween(3, 7): {string.Join(", ", view)}");
set.Add(6);
Console.WriteLine($"the same view after set.Add(6): {string.Join(", ", view)}");

var queue = new Queue<string>();
queue.Enqueue("a");
queue.Enqueue("b");
Console.WriteLine($"Queue: {queue.Dequeue()} then {queue.Peek()}");

var both = new HashSet<int> { 1, 2, 3, 4 };
Console.WriteLine($"Add(3) on a set holding 3 returns: {both.Add(3)}");
Console.WriteLine($"LINQ Intersect: {string.Join(", ", both.Intersect(new[] { 3, 4, 5 }))}, the set still has {both.Count} items");

var left = new HashSet<int> { 1, 2, 3, 4 };
left.IntersectWith(new[] { 3, 4, 5 });
Console.WriteLine($"after left.IntersectWith, left is: {string.Join(", ", left)}");
