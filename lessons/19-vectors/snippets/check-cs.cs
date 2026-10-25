using System.Runtime.InteropServices;

void Must(bool ok, string what)
{
    if (!ok) throw new Exception(what);
}

var v = new List<int> { 3, 1, 2 };
v.Add(4);
v.Insert(1, 5);
v.AddRange(new[] { 7, 8 });
Must(v.Count == 7 && v.Contains(7), "add insert count contains");
v.RemoveAt(1);
Must(v[^1] == 8, "index from end");
v.RemoveAll(x => x == 7);
Must(v.SequenceEqual(new[] { 3, 1, 2, 4, 8 }), "RemoveAll");

v.Sort();
Must(v.SequenceEqual(new[] { 1, 2, 3, 4, 8 }), "Sort");
Must(v.BinarySearch(3) == 2, "BinarySearch hit");
Must(v.BinarySearch(5) == ~4, "BinarySearch miss is a negative number");
v.Sort((a, b) => b - a);
Must(v.SequenceEqual(new[] { 8, 4, 3, 2, 1 }), "Sort with comparison");

var copy = v.GetRange(1, 2);
copy[0] = 99;
Must(v[1] == 4, "GetRange copies");
var view = CollectionsMarshal.AsSpan(v)[1..3];
view[0] = 77;
Must(v[1] == 77, "AsSpan is a view");
Console.WriteLine("all checks passed");
