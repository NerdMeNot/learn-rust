var found = FindUser(1);
if (found is not null)
{
    Console.WriteLine($"found {found.Name}");
}
else
{
    Console.WriteLine("no user");
}

// pattern matching does the check and the unwrap in one step
if (FindUser(2) is { } u)
{
    Console.WriteLine($"found {u.Name}");
}
else
{
    Console.WriteLine("no user");
}

// User? means "a User, or null"
static User? FindUser(uint id) => id == 1 ? new User("ana") : null;

record User(string Name);
