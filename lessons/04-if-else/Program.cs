var n = 7;
if (n % 2 == 0)
{
    Console.WriteLine($"{n} is even");
}
else
{
    Console.WriteLine($"{n} is odd");
}

// the ?: operator gives back a value
var size = n > 5 ? "big" : "small";
Console.WriteLine(size);
