Counter.Hits = 5;
Console.WriteLine("main thread sees: " + Counter.Hits);

var other = new Thread(() => Console.WriteLine("second thread sees: " + Counter.Hits));
other.Start();
other.Join();

class Counter
{
    [ThreadStatic] public static int Hits; // one copy per thread
}
