using System.Collections.Generic;

var list = new List<Person> { new Person { Age = 36 } };
foreach (var p in list)
{
    p.Age++;
}
list[0].Age++;

struct Person { public int Age; }
