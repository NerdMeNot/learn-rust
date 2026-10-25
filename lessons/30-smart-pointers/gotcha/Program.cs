var a = new Counter();
var b = a;

a.Hits++;
b.Hits++;
Console.WriteLine($"hits: {a.Hits}");

class Counter
{
    public int Hits;
}
