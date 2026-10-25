static void Log(string msg) => Console.WriteLine("log: " + msg);

var r = Log("hello");
Console.WriteLine(r);
