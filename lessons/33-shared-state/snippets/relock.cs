var gate = new object();

void Inner()
{
    lock (gate)
    {
        Console.WriteLine("inner: same thread, lock already held, not blocked");
    }
}

lock (gate)
{
    Inner();
}
