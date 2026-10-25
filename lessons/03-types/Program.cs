int a = 7;          // 32-bit signed integer
long b = 10;        // 64-bit signed integer
double price = 2.5; // 64-bit float
bool ok = true;

long total = a + b;          // int widens to long on its own
double cost = a * price;     // and to double too
Console.WriteLine($"{total} {cost} {ok.ToString().ToLower()}");
