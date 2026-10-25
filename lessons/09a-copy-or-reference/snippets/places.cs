using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

var structs = new[] { new SPerson { Age = 36 }, new SPerson { Age = 54 } };
foreach (var p in structs)
{
    var copy = p; // the loop variable is read-only, so copy it to change anything
    copy.Age++;
}
Console.WriteLine($"structs after foreach and a copy: {structs[0].Age}, {structs[1].Age}");

structs[0].Age++; // an array element is a variable: this changes it in place
Console.WriteLine($"structs after structs[0].Age++:   {structs[0].Age}, {structs[1].Age}");

var structList = new List<SPerson> { new SPerson { Age = 36 } };
foreach (ref var p in CollectionsMarshal.AsSpan(structList))
{
    p.Age++; // ref foreach over a Span gives the element itself
}
Console.WriteLine($"List<struct> after a ref foreach over its Span: {structList[0].Age}");

var classes = new List<CPerson> { new CPerson { Age = 36 }, new CPerson { Age = 54 } };
foreach (var p in classes)
{
    p.Age++; // the loop variable names the object: this changes it
}
Console.WriteLine($"classes after foreach p.Age++:    {classes[0].Age}, {classes[1].Age}");

var one = new SPerson { Age = 1 };
Birthday(one);
Console.WriteLine($"struct after Birthday(one):        {one.Age}");
BirthdayRef(ref one);
Console.WriteLine($"struct after BirthdayRef(ref one): {one.Age}");

var cp = new CPerson { Age = 1 };
BirthdayClass(cp);
Console.WriteLine($"class after BirthdayClass(cp):    {cp.Age}");

// A lambda captures the variable itself, so it can change it
int count = 0;
Action inc = () => count++;
inc();
inc();
Console.WriteLine($"count after two inc() calls:      {count}");

static void Birthday(SPerson p) { p.Age++; }
static void BirthdayRef(ref SPerson p) { p.Age++; }
static void BirthdayClass(CPerson p) { p.Age++; }

struct SPerson { public int Age; }
class CPerson { public int Age; }
