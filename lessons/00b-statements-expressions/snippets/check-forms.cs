static void Log(string msg) => Console.WriteLine("log: " + msg);

int n = 7;

string size = n > 5 ? "big" : "small";
Console.WriteLine($"ternary: {size}");

int area;
{
    int w = 3;
    int h = 4;
    area = w * h;
}
Console.WriteLine($"block: {area}");

string name = 2 switch
{
    1 => "one",
    2 => "two",
    _ => "many",
};
Console.WriteLine($"switch: {name}");

int a = 1, b = 2;
(a, b) = (b, a);
Console.WriteLine($"swap: {a} {b}");

Log("first");
int c = 2;
Console.WriteLine($"comma: {c}");

Func<int, int> addOne = x =>
{
    int y = x + 1;
    return y;
};
Func<int, int> expr = x => x + 1;
Console.WriteLine($"closure: {addOne(41)} {expr(41)}");
