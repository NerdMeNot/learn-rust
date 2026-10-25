var factor = 3;
var times = (int x) => x * factor;   // reads factor
Console.WriteLine(times(5));

var count = 0;
var inc = () => count++;             // changes count
inc();
inc();
Console.WriteLine($"count={count}");
