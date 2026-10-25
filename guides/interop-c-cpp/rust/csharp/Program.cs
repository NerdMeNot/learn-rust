using System.Runtime.InteropServices;

Console.WriteLine($"distance: {Native.point_distance(new Point { x = 0, y = 0 }, new Point { x = 3, y = 4 })}");

IntPtr text = Native.greeting("C#");
Console.WriteLine(Marshal.PtrToStringUTF8(text));
Native.greeting_free(text);

[StructLayout(LayoutKind.Sequential)]
struct Point { public double x; public double y; }

static class Native
{
    [DllImport("bridge")]
    public static extern double point_distance(Point a, Point b);

    [DllImport("bridge")]
    public static extern IntPtr greeting([MarshalAs(UnmanagedType.LPUTF8Str)] string name);

    [DllImport("bridge")]
    public static extern void greeting_free(IntPtr text);
}
