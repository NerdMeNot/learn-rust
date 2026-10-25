using System;
using System.Collections.Generic;

Console.WriteLine("List<int> is List<string>: " + (typeof(List<int>) == typeof(List<string>)));
Console.WriteLine("type of a List<int>: " + new List<int>().GetType());
Console.WriteLine("T in Name<int>(): " + Name<int>());
Console.WriteLine("T in Name<string>(): " + Name<string>());
Console.WriteLine("new T[2] has length " + Make<string>().Length);

// T is a real type while the program runs.
static string Name<T>() => typeof(T).Name;

static T[] Make<T>() => new T[2];
