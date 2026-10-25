var baseConfig = new Config("service", 8080, false);
var test = baseConfig with { Port = 9000 };
Console.WriteLine(baseConfig);
Console.WriteLine(test);

record Config(string Name, int Port, bool Verbose);
