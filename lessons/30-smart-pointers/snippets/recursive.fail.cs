// a struct stores its fields inline, so it cannot contain itself
struct Expr
{
    public int Value;
    public Expr Left;
    public Expr Right;
}

class Program
{
    static void Main()
    {
        var e = new Expr();
        System.Console.WriteLine(e.Value);
    }
}
