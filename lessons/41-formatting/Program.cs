var n = 42;
Console.WriteLine($"[{n,6}] [{n,-6}] [{n:D6}]"); // no centering
Console.WriteLine($"{3.14159:F2} {1234.5:e} 0x{n:x} 0b{n:b}");

var price = new Money(1999);
Console.WriteLine($"total: {price}");

var label = $"{price} each";
Console.WriteLine($"{label} ({label.Length} chars)");

var item = new Item("pen", 3);
Console.WriteLine($"{item.Qty} x {item.Name}");
Console.WriteLine(item); // records write ToString for you

// ToString is C#'s Display
record Money(long Cents)
{
    public override string ToString() => $"${Cents / 100}.{Cents % 100:D2}";
}

record Item(string Name, int Qty);
