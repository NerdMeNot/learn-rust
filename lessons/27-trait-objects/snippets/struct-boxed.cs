using System;
using System.Collections.Generic;

var sq = new Sq { Side = 3 };
var list = new List<IShape> { sq }; // putting a struct behind an interface boxes a copy
list[0].Grow();
Console.WriteLine($"original side: {sq.Side}, boxed copy side: {((Sq)list[0]).Side}");

GrowGeneric(ref sq); // a struct as the type argument: no box
Console.WriteLine($"after the generic call, original side: {sq.Side}");

static void GrowGeneric<T>(ref T s) where T : IShape => s.Grow();

interface IShape
{
    void Grow();
}

struct Sq : IShape
{
    public int Side;
    public void Grow() => Side++;
}
