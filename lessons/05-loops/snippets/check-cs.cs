using System.Linq;

void Must(bool ok, string what)
{
    if (!ok) throw new Exception(what);
}

var sum = 0;
foreach (var x in new[] { 1, 2, 3 }) sum += x;
Must(sum == 6, "foreach");

// Enumerable.Range takes a start and a COUNT
Must(string.Join(",", Enumerable.Range(1, 3)) == "1,2,3", "Range(1, 3)");

var n = 0;
do { n += 5; } while (n < 3);
Must(n == 5, "do-while runs once");

// break inside a switch leaves the switch only
var printed = 0;
for (var v = 1; v <= 5; v++)
{
    switch (v)
    {
        case 3:
            break;
        default:
            printed++;
            break;
    }
}
Must(printed == 4, "break in switch");

// continue in a for runs the step; in a do-while it goes to the test
var steps = 0;
for (var i = 0; i < 3; i++)
{
    if (i == 1) continue;
    steps++;
}
Must(steps == 2, "continue runs i++");
var k = 0;
var passes = 0;
do
{
    k++;
    passes++;
    if (k >= 3) continue;
} while (k < 3);
Must(k == 3 && passes == 3, "continue in do-while");
Console.WriteLine("ok");
