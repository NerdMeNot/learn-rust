using System.Linq;

void Must(bool ok, string what)
{
    if (!ok) throw new Exception(what);
}

int Add(int a, int b) => a + b;

const int Max = 100;
int count = 5;
var inferred = 5;
int[] arr = { 1, 2, 3 };
Must(count == inferred && Max == 100 && Add(1, 2) == 3 && arr.Length == 3, "declarations");

var (a, b) = (1, "two");
Must(a == 1 && b == "two", "tuple");

double price = 2.9;
int n = 7;
Must((int)price == 2 && (int)-price == -2 && (double)n == 7.0, "casts");

Func<int, int> inc = x => x + 1;
Must(inc(1) == 2, "lambda");

var empty = Enumerable.Empty<string>();
Must(!empty.Any(), "explicit type argument");

string verbatim = @"C:\temp";
Must(verbatim.Length == 7, "verbatim");
Must(1_000 == 1000 && 0xFF == 255 && 0b101 == 5 && 5u == 5 && 5L == 5 && 2.5f == 2.5, "literals");
char c = 'R';
Must(c == 82 && sizeof(char) == 2, "char is 2 bytes");
