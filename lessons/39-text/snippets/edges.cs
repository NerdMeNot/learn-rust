using System.Globalization;

Console.WriteLine("\"a,b,\".Split(','):  " + string.Join("|", "a,b,".Split(',')) + "  (" + "a,b,".Split(',').Length + " pieces)");
Console.WriteLine("\"\".Split(',').Length:  " + "".Split(',').Length);
Console.WriteLine("RemoveEmptyEntries:  " + "a,,b,".Split(',', StringSplitOptions.RemoveEmptyEntries).Length + " pieces");
Console.WriteLine("Split(',', 2):       " + string.Join("|", "k,v,w".Split(',', 2)));

foreach (var text in new[] { "42", "+42", " 42", "42 ", "1_000" })
{
    Console.WriteLine($"int.TryParse(\"{text}\"): {int.TryParse(text, out var n)} {n}");
}

// The default number styles allow thousands separators, so the comma is not a decimal mark here.
Console.WriteLine("double.Parse(\"1,5\", InvariantCulture): " + double.Parse("1,5", CultureInfo.InvariantCulture));
Console.WriteLine("current culture name: \"" + CultureInfo.CurrentCulture.Name + "\"");

Console.WriteLine("\"straße\".ToUpperInvariant(): " + "straße".ToUpperInvariant());
Console.WriteLine("Equals(\"É\", \"é\", OrdinalIgnoreCase): " + string.Equals("É", "é", StringComparison.OrdinalIgnoreCase));
