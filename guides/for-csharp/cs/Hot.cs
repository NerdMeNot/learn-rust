static async Task<int> Save()
{
    Console.WriteLine("saving");
    await Task.Delay(1);
    return 42;
}

var task = Save();               // runs up to its first await that has to wait, right now
Console.WriteLine("called Save");
Console.WriteLine(await task);
