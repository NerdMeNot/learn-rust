var u = new User("ana", 30, true);
Console.WriteLine($"{u.Name} is {u.Age}, active={u.Active}");
Console.WriteLine(u);   // records print their fields for free

record User(string Name, uint Age, bool Active);
