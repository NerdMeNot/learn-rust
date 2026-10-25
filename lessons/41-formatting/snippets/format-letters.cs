using System;

int n = 5;
try { Console.WriteLine($"{n:Q}"); } catch (FormatException e) { Console.WriteLine("FormatException: " + e.Message); }
Console.WriteLine($"{"text":D}");
try { Console.WriteLine(string.Format("{0} {1}", 1)); } catch (FormatException e) { Console.WriteLine("FormatException: " + e.Message); }
try { Console.WriteLine(string.Format("{0:Q}", n)); } catch (FormatException e) { Console.WriteLine("FormatException: " + e.Message); }
