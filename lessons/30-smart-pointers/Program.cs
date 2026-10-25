// every object is already a shared reference
var log = new List<string>();
var other = log;

log.Add("from log");
other.Add("from other");

Console.WriteLine($"log: {string.Join(", ", log)}");
var same = ReferenceEquals(log, other) ? "true" : "false";
Console.WriteLine($"same list: {same}");
