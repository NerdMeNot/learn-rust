var a = new Team { Name = "red", Members = new List<string> { "ana" } };
var b = a;               // a struct is copied...
b.Name = "blue";
b.Members.Add("bo");     // ...but the List inside it is not
Console.WriteLine($"{a.Name} {a.Members.Count}");
Console.WriteLine($"{b.Name} {b.Members.Count}");

struct Team
{
    public string Name;
    public List<string> Members;
}
