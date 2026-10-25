int? age = null;

// The operators are "lifted": they compile, and a null gives null (or false).
int? next = age + 1;
Console.WriteLine($"age + 1 is null: {next is null}");
Console.WriteLine($"age > 3: {age > 3}");
Console.WriteLine($"age == 3: {age == 3}");
Console.WriteLine($"age < 3: {age < 3}");
