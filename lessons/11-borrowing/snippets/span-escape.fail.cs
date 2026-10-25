Span<int> Make()
{
    Span<int> s = stackalloc int[3];
    return s; // points at memory that is gone when Make returns
}

Console.WriteLine(Make().Length);
