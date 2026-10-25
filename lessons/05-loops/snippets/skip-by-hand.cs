var passes = 0;
for (var i = 0; i < 5; i++)
{
    if (i == 1)
    {
        i += 2;
    }
    passes++;
    Console.WriteLine($"i: {i}");
}
Console.WriteLine($"passes: {passes}");
