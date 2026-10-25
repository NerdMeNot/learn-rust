static int Clamp(int x)
{
    if (x < 0)
    {
        return 0;
    }
    return x > 100 ? 100 : x;
}

int n = 7;
string label = n % 2 == 0 ? "even" : "odd";
int w = 3;
int h = 4;
int area = w * h;
Console.WriteLine($"{n} is {label}");
Console.WriteLine($"area: {area}");
Console.WriteLine($"clamp: {Clamp(-5)} {Clamp(50)} {Clamp(250)}");
