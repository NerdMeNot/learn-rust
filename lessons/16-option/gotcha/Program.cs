var u = FindUser(2);
Console.WriteLine(u.Name);

static User? FindUser(uint id) => id == 1 ? new User("ana") : null;

record User(string Name);
