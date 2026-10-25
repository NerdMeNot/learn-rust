Console.WriteLine(BadSpan().Length);
Console.WriteLine(BadRef());

static Span<int> BadSpan()
{
    Span<int> s = stackalloc int[4];
    return s;
}

static ref int BadRef()
{
    int x = 1;
    return ref x;
}
