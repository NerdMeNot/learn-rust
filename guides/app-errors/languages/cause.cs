using System;
try
{
    try { int.Parse("eighty"); }
    catch (FormatException e) { throw new Exception("reading bad.conf", e); }
}
catch (Exception e)
{
    Console.WriteLine($"{e.Message}: {e.InnerException!.Message}");
}
