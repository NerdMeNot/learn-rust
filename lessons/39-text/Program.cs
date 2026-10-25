using System.Text;

var line = "  name=Ana, age=30, city=Lisbon  ";

var clean = line.Trim(); // a new string: .NET strings never share a view
Console.WriteLine($"[{clean}]");

// split on ',', trim each piece, split once on '='
foreach (var field in clean.Split(','))
{
    var kv = field.Trim().Split('=', 2);
    if (kv.Length == 2)
    {
        Console.WriteLine($"{kv[0],5} -> {kv[1]}");
    }
}

// text to number: TryParse reports success with a bool
var age = int.TryParse("30", out var n) ? n : 0;
Console.WriteLine($"next year: {age + 1}");

var shout = clean.ToUpperInvariant().Replace(", ", " | ");
Console.WriteLine(shout);

Console.WriteLine($"starts with name: {clean.StartsWith("name", StringComparison.Ordinal)}");
Console.WriteLine($"position of age: {clean.IndexOf("age", StringComparison.Ordinal)}"); // -1 if missing

Console.WriteLine(string.Join("-", "build", "with", "rust"));

// grow a string piece by piece
var sb = new StringBuilder();
for (var i = 1; i <= 3; i++)
{
    sb.Append(i).Append(';');
}
Console.WriteLine(sb);
