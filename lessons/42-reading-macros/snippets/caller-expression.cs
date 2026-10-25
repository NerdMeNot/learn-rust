using System.Runtime.CompilerServices;

static void Check(bool condition, [CallerArgumentExpression("condition")] string? text = null)
{
    Console.WriteLine($"{(condition ? "passed" : "failed")}: {text}");
}

int a = 1, b = 2;
Check(a + b == 3);
Check(a > b);
