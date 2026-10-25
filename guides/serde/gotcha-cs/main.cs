using System.Text.Json;

var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
var text = """{ "fullName": "Cy" }"""; // no "age"

var cy = JsonSerializer.Deserialize<Person>(text, options)!;
Console.WriteLine($"{cy.FullName} is {cy.Age}");

try
{
    JsonSerializer.Deserialize<RequiredPerson>(text, options);
}
catch (JsonException e)
{
    Console.WriteLine("JsonException: " + e.Message);
}

class Person
{
    public string FullName { get; set; } = "";
    public int Age { get; set; }
}

class RequiredPerson
{
    public required string FullName { get; set; }
    public required int Age { get; set; }
}
