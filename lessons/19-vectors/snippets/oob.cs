var list = new List<int> { 10, 20, 30 };
int[] array = { 10, 20, 30 };

try
{
    Console.WriteLine(list[10]);
}
catch (Exception e)
{
    Console.WriteLine($"List[10]: {e.GetType().Name}");
}

try
{
    Console.WriteLine(array[10]);
}
catch (Exception e)
{
    Console.WriteLine($"array[10]: {e.GetType().Name}");
}

Console.WriteLine($"ElementAtOrDefault(10): {list.ElementAtOrDefault(10)}");
