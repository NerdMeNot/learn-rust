using System.Runtime.InteropServices;

// no `unsafe` keyword and no project switch: the runtime converts the string
Console.WriteLine($"strlen(\"héllo\") = {Native.strlen("héllo")} bytes");

static class Native
{
    [DllImport("libc.so.6")]
    public static extern nuint strlen([MarshalAs(UnmanagedType.LPUTF8Str)] string s);
}
