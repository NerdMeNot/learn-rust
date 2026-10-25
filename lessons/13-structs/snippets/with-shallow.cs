var a = new Team("red", new List<string> { "ana" });
var b = a with { Name = "blue" };
b.Members.Add("bob");
Console.WriteLine($"a members: {string.Join(", ", a.Members)}");
Console.WriteLine($"b members: {string.Join(", ", b.Members)}");
Console.WriteLine($"same list object: {ReferenceEquals(a.Members, b.Members)}");

record Team(string Name, List<string> Members);
