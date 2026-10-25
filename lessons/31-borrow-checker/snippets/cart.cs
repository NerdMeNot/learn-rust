var cart = new Cart();
var view = cart.Items;
view.Add("tea");
Console.WriteLine($"cart size: {cart.Size}");

class Cart
{
    private readonly List<string> items = new();

    public List<string> Items => items;

    public int Size => items.Count;
}
