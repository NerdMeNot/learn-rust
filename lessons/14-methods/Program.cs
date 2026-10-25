var c = new Counter();
c.Bump();
c.Bump();
Console.WriteLine(c.Get());

class Counter
{
    private uint count;   // starts at 0

    public uint Get() => count;

    public void Bump() => count++;
}
