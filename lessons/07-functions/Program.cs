Console.WriteLine(Add(2, 3));
Greet("kotlin");

static int Add(int a, int b) => a + b;

static void Greet(string name)   // void: returns nothing
{
    Console.WriteLine($"hi {name}");
}
