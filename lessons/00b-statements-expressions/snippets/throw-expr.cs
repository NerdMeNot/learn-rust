bool ok = args.Length == 0;
int x = ok ? 5 : throw new InvalidOperationException("not ok");
Console.WriteLine($"x: {x}");
