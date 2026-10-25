foreach (var p in typeof(Person).GetProperties())
    Console.WriteLine($"{p.Name}: {p.PropertyType.Name}");

class Person
{
    public string Name { get; set; } = "";
    public int Age { get; set; }
}
