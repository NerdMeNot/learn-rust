using (var a = new Noisy("a"))
using (var b = new Noisy("b"))
{
    Console.WriteLine("  end of block");
}

class Noisy(string name) : IDisposable
{
    public void Dispose() => Console.WriteLine($"  dispose {name}");
}
