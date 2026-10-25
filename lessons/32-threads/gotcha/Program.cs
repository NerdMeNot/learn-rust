string[] names = ["a", "b", "c"];
var t = new Thread(() =>
{
    Console.WriteLine(names.Length);
});
t.Start();
t.Join();
