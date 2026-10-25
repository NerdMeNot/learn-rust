// Backs the C# cells of the tables in lesson.mdx. Never shown.
var v = new List<string> { "ana", "bo", "cy" };
var withIndex = v.Select((x, i) => $"{i}{x}").ToList();
if (string.Join(",", withIndex) != "0ana,1bo,2cy") throw new Exception("select");
if (v.Zip(new[] { 31, 27 }).Count() != 2) throw new Exception("zip");
if (string.Join(",", Enumerable.Reverse(v)) != "cy,bo,ana") throw new Exception("reverse");
var chunks = Enumerable.Range(1, 5).Chunk(2).ToList();
if (chunks.Count != 3 || chunks[2].Length != 1) throw new Exception("chunk");
var nums = new List<int> { 1, 2, 3, 4 };
nums.RemoveAll(x => x % 2 == 0);
if (string.Join(",", nums) != "1,3") throw new Exception("removeall");
var grow = new List<int> { 1, 2, 3, 4 };
try
{
    foreach (var x in grow)
    {
        if (x == 2) grow.Add(99);
    }
    throw new Exception("no exception");
}
catch (InvalidOperationException)
{
}
Console.WriteLine("ok");
