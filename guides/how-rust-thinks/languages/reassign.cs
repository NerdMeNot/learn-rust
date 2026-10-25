var count = 0;
count += 1; // fine: locals are mutable unless you write `const`
const int Fixed = 0;
Fixed += 1;
Console.WriteLine(count);
