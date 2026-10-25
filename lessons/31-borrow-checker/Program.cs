var inv = new Inventory([new Item("bolt", 0), new Item("nut", 12), new Item("washer", 0)]);
inv.Check();
Console.WriteLine("log: [" + string.Join(", ", inv.Log.Select(s => $"\"{s}\"")) + "]");

class Item(string name, int count)
{
    public string Name { get; } = name;
    public int Count { get; } = count;
}

class Inventory(List<Item> items)
{
    public List<string> Log { get; } = new();

    public void Check()
    {
        foreach (var item in items)
        {
            if (item.Count == 0)
            {
                Log.Add($"out of stock: {item.Name}");
            }
        }
    }
}
