var order = new OrderId(7);
Console.WriteLine(LoadUser(order));

static string LoadUser(UserId id) => $"user #{id.Value}";

readonly record struct UserId(int Value);
readonly record struct OrderId(int Value);
