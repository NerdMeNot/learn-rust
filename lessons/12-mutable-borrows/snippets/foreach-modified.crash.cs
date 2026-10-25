var list = new List<int> { 1, 2, 3, 4 };
foreach (var x in list)
{
    Console.WriteLine($"saw {x}");
    if (x == 2) list.Add(99); // changes the list the loop is walking
}
