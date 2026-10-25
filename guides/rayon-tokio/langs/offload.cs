using System;
using System.Linq;
using System.Threading.Tasks;

var total = await Task.Run(() => Enumerable.Range(0, 1_000_000).AsParallel().Sum(n => (long)(n % 7)));
Console.WriteLine($"Task.Run + PLINQ: {total}");
