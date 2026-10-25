static string Show<T>(T x) => $"{typeof(T).Name}: {x}";

Console.WriteLine(Show(5));
Console.WriteLine(Show("hi"));
