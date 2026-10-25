object x = "hello";
if (x is string s)
{
    Console.WriteLine("text of length " + s.Length);
}
int n = 7;
string size = n > 5 ? "big" : "small";
Console.WriteLine(size);
