using (var a = new Guard("a"))
using (var b = new Guard("b"))
{
    Console.WriteLine("working");
}

var c = new Guard("c");          // no `using`: nothing ever calls Dispose
Console.WriteLine("end of Main");

class Guard : IDisposable
{
    private readonly string name;

    public Guard(string name)
    {
        this.name = name;
        Console.WriteLine($"open {name}");
    }

    public void Dispose() => Console.WriteLine($"close {name}");
}
