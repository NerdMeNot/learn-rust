using System.Runtime.CompilerServices;

var weak = MakeCycle();
GC.Collect();
GC.WaitForPendingFinalizers();
GC.Collect();
Console.WriteLine($"cycle collected: {!weak.IsAlive}");

[MethodImpl(MethodImplOptions.NoInlining)]
static WeakReference MakeCycle()
{
    var a = new Node();
    var b = new Node();
    a.Other = b;
    b.Other = a;
    return new WeakReference(a); // does not keep a alive
}

class Node
{
    public Node? Other;
}
