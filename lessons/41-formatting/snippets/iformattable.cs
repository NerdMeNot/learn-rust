using System;

var t = new Temp(21.456);
Console.WriteLine($"{t}");
Console.WriteLine($"{t:F1}");
Console.WriteLine($"{t:K}");
Console.WriteLine($"[{t,12:F1}]");

class Temp(double celsius) : IFormattable
{
    public string ToString(string? format, IFormatProvider? provider) => format switch
    {
        "K" => (celsius + 273.15).ToString("F2") + " K",
        null or "" => celsius.ToString() + " C",
        _ => celsius.ToString(format, provider) + " C",
    };

    public override string ToString() => ToString(null, null);
}
