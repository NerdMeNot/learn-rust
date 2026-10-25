int[] array = { 10, 20, 30, 40 };

int[] copy = array[1..3];          // a range on an array makes a new array
copy[0] = 99;
Console.WriteLine($"after copy[0] = 99:  array = [{string.Join(", ", array)}]");

Span<int> view = array.AsSpan(1, 2); // a view of the same memory
view[0] = 99;
Console.WriteLine($"after view[0] = 99:  array = [{string.Join(", ", array)}]");
