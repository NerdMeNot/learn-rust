var data = new List<int> { 1, 2, 3 };
var gate = new object();

var worker = new Thread(() =>
{
    try
    {
        lock (gate)
        {
            data.Add(4);
            throw new InvalidOperationException("worker failed halfway");
        }
    }
    catch (InvalidOperationException e)
    {
        Console.WriteLine("worker died: " + e.Message);
    }
});
worker.Start();
worker.Join();

lock (gate) // not poisoned: leaving the block released the lock
{
    Console.WriteLine("main got the lock; data: [" + string.Join(", ", data) + "]");
}
