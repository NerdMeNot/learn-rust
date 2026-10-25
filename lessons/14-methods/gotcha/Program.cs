var c = new Counter();
c.Bump();
Console.WriteLine(c.Count);

class Counter
{
    public uint Count { get; private set; }

    public void Bump() => Count++;
}
