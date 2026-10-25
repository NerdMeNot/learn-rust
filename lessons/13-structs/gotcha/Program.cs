var u = new User { Name = "ana" };
Console.WriteLine($"{u.Name} {u.Age} {u.Active}");

class User
{
    public string Name { get; set; } = "";
    public uint Age { get; set; }
    public bool Active { get; set; }
}
