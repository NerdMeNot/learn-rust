var gate = new object();
Monitor.Enter(gate); // taken on the main thread

var other = new Thread(() =>
{
    try
    {
        Monitor.Exit(gate); // released from a different thread
    }
    catch (SynchronizationLockException e)
    {
        Console.WriteLine("other thread: " + e.GetType().Name);
    }
});
other.Start();
other.Join();
Console.WriteLine("held by main still: " + Monitor.IsEntered(gate));
