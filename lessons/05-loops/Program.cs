// 1, 2, 3
for (var i = 1; i < 4; i++)
{
    Console.WriteLine($"for {i}");
}

var n = 3;
while (n > 0)
{
    n--;
}

// C# break can't hand back a value, so keep it in a variable
var tries = 0;
int found;
while (true)
{
    tries++;
    if (tries == 3)
    {
        found = tries * 10;
        break;
    }
}
Console.WriteLine($"n={n} found={found}");
