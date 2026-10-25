int max = int.MaxValue;
Console.WriteLine($"MaxValue + 1: {max + 1}");

try
{
    Console.WriteLine(checked(max + 1));
}
catch (OverflowException e)
{
    Console.WriteLine($"checked: {e.Message}");
}

int n = 300;
Console.WriteLine($"(byte) 300: {unchecked((byte)n)}");
byte b1 = 100, b2 = 100;
var sum = b1 + b2;
Console.WriteLine($"byte + byte: {sum} ({sum.GetType().Name})");

int big = 16_777_217;
float f = big;
Console.WriteLine($"int 16777217 to float: {(int)f}");
Console.WriteLine($"-7 / 2: {-7 / 2}");
Console.WriteLine($"-7 % 2: {-7 % 2}");
Console.WriteLine($"0.1m + 0.2m: {0.1m + 0.2m}");
Console.WriteLine($"\"😀\".Length: {"😀".Length}");
Console.WriteLine($"int.TryParse(\" 12 \"): {int.TryParse(" 12 ", out var v)} {v}");
