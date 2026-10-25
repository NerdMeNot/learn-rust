using System;

class Span : IDisposable
{
    private readonly string name;

    public Span(string name)
    {
        this.name = name;
        Console.WriteLine("enter " + name);
    }

    public void Dispose() => Console.WriteLine("exit " + name);
}

class Program
{
    static void Main()
    {
        using (var span = new Span("request"))
        {
            Console.WriteLine("listed notes"); // Dispose() runs when the block ends
        }
        _ = new Span("forgotten"); // `_` discards it; nothing ever calls Dispose
        Console.WriteLine("after");
    }
}
