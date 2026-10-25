using System.Numerics;

Console.WriteLine(Add(2, 3));
Console.WriteLine(Add(1.5, 2.25));

// INumber<T> (C# 11) says "T has + and gives a T back".
static T Add<T>(T a, T b) where T : INumber<T>
{
    return a + b;
}
