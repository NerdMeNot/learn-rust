// 1. init-only properties with defaults, and `with`: the nearest to a builder and to ..Default::default().
var plain = new Config();
var custom = new Config { Port = 9000, Verbose = true };
var moved = plain with { Port = 9001 };
Console.WriteLine(plain);
Console.WriteLine(custom);
Console.WriteLine(moved);

// 2. A record struct is a distinct type around one value.
Console.WriteLine(new UserId(7));

record Config
{
    public string Host { get; init; } = "localhost";
    public int Port { get; init; } = 8080;
    public int Retries { get; init; } = 3;
    public bool Verbose { get; init; }
}

readonly record struct UserId(int Value);
