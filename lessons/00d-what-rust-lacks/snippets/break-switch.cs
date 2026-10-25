var n = 0;
for (var i = 0; i < 5; i++)
{
    switch (i)
    {
        case 2:
            break;
        default:
            n++;
            break;
    }
}
Console.WriteLine($"n: {n}");
