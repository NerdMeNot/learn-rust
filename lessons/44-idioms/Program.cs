// The constructor is private, so Port.Parse is the only way in.
class Port
{
    public int Number { get; }

    private Port(int number) { Number = number; }

    public static Port Parse(string text)
    {
        if (!long.TryParse(text, out var n)) throw new FormatException("not a number");
        if (n < 1 || n > 65535) throw new FormatException($"{n} is outside 1-65535");
        return new Port((int)n);
    }
}

class Program
{
    static void Listen(Port port) => Console.WriteLine($"listening on {port.Number}");

    static void Main()
    {
        foreach (var text in new[] { "8080", "0", "http" })
        {
            try { Listen(Port.Parse(text)); }
            catch (FormatException e) { Console.WriteLine($"\"{text}\": {e.Message}"); }
        }
    }
}
