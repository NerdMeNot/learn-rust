using System;

// C# allows one class to implement IComparable<T> for two different T.
Console.WriteLine("compared with an int: " + new Length().CompareTo(5));
Console.WriteLine("compared with a string: " + new Length().CompareTo("abc"));

class Length : IComparable<int>, IComparable<string>
{
    public int CompareTo(int other) => 1;

    public int CompareTo(string? other) => 2;
}
