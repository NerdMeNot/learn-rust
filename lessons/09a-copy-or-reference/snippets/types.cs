using System;
using System.Collections.Generic;

void Report(string kind, bool changed) =>
    Console.WriteLine($"{kind,-8} {(changed ? "shares: the other name sees the change" : "copy:   the other name is untouched")}");

int n = 1;
int m = n;
m = 99;
Report("int", n == 99);

SPoint s1 = new SPoint { X = 1 };
SPoint s2 = s1;
s2.X = 99;
Report("struct", s1.X == 99);

CPoint c1 = new CPoint { X = 1 };
CPoint c2 = c1;
c2.X = 99;
Report("class", c1.X == 99);

int[] arr = { 1, 2 };
int[] arr2 = arr;
arr2[0] = 99;
Report("int[]", arr[0] == 99);

var list = new List<SPoint> { new SPoint { X = 1 } };
var list2 = list;
list2.Add(new SPoint { X = 2 });
Report("List", list.Count == 2);

// Identity and equality are different questions
CPoint c3 = new CPoint { X = 99 };
Console.WriteLine($"ReferenceEquals(c1, c2): {ReferenceEquals(c1, c2)}  (same object)");
Console.WriteLine($"ReferenceEquals(c1, c3): {ReferenceEquals(c1, c3)}  (same X, different object)");
RPoint r1 = new RPoint(99); // a record is a class with value equality
RPoint r2 = new RPoint(99);
Console.WriteLine($"r1 == r2: {r1 == r2}  (a record compares values)");
Console.WriteLine($"ReferenceEquals(r1, r2): {ReferenceEquals(r1, r2)}");

RSPoint rs1 = new RSPoint(1);
RSPoint rs2 = rs1 with { X = 9 }; // a record struct is a value: this is a changed copy
Console.WriteLine($"record struct: rs1.X = {rs1.X}, rs2.X = {rs2.X}");

struct SPoint { public int X; }
class CPoint { public int X; }
record RPoint(int X);
record struct RSPoint(int X);
