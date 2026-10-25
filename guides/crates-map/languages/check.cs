using System;
using System.Linq;
using System.Text.RegularExpressions;

var uuid = Guid.Parse("67e55044-10b1-426f-9247-bb680e5fe0c8");
var json = System.Text.Json.JsonSerializer.Serialize(new { a = 1 });
var m = Regex.Match("shipped on 2026-10-25", @"\d{4}-\d{2}-\d{2}");
var total = 0;
System.Threading.Tasks.Parallel.For(0, 10, i => System.Threading.Interlocked.Add(ref total, i));
Console.WriteLine($"C#: Guid {uuid.ToString()[..8]}, JsonSerializer {json}, Regex {m.Value}, Parallel.For total {total}, PLINQ total {Enumerable.Range(0, 10).AsParallel().Sum()}, HttpClient {typeof(System.Net.Http.HttpClient).Name}, DateTime {new DateTime(2026, 10, 25).DayOfWeek}");
