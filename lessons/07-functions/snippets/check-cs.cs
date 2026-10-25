static void Must(bool ok, string what)
{
    if (!ok) throw new Exception(what);
}

static int TotalWithTax(int price)
{
    int rate = 20;
    int Tax(int p) => p * rate / 100; // a local function may use `rate`
    return price + Tax(price);
}

static int Area(int w, int h = 1) => w * h;

static void DivMod(int a, int b, out int q, out int r)
{
    q = a / b;
    r = a % b;
}

static int Sum(params int[] xs)
{
    int total = 0;
    foreach (var x in xs) total += x;
    return total;
}

static (int, int) DivMod2(int a, int b) => (a / b, a % b);

static void Fill(List<string> xs)
{
    xs.Add("x");
}

static int Bump(int n)
{
    n += 1; // a parameter can be reassigned
    return n;
}

static int Twice(int n) => n * 2;

Must(TotalWithTax(50) == 60, "local function captures");
Must(Area(3) == 3 && Area(3, 4) == 12 && Area(w: 2, h: 5) == 10, "default and named");
DivMod(7, 2, out int q, out int r);
Must(q == 3 && r == 1, "out parameters");
Must(Sum(1, 2, 3) == 6, "params");
var (q2, r2) = DivMod2(7, 2);
Must(q2 == 3 && r2 == 1, "tuple");
var xs = new List<string>();
Fill(xs);
Must(xs.Count == 1, "the caller sees the callee's change");
Must(Bump(1) == 2, "reassign parameter");
Func<int, int> f = Twice;
Must(f(4) == 8, "method group");
Console.WriteLine("all checks passed");
