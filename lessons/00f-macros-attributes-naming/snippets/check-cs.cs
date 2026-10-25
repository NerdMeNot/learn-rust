var bad = false;

void Same(string got, string want)
{
    if (got != want)
    {
        Console.WriteLine($"got [{got}], want [{want}]");
        bad = true;
    }
}

Same($"{42,5}", "   42");
Same($"{42,-5}", "42   ");
Same($"{42:D5}", "00042");
Same($"{3.14159:F2}", "3.14");
Same($"{3.14159,8:F3}", "   3.142");
Same($"{255:x}", "ff");
Same($"{"Ada",10}", "       Ada");
Same($"{"Ada",-10}", "Ada       ");
Same($"{42} {"Ada"}", "42 Ada");
Same($"{{}}", "{}");
if (bad) throw new Exception("a C# format row is wrong");
Console.WriteLine("ok");
