Color c = (Color)42; // compiles, and runs
Console.WriteLine($"(Color)42 prints as: {c}");
Console.WriteLine($"Enum.IsDefined: {Enum.IsDefined(c)}");

enum Color { Red, Green, Blue }
