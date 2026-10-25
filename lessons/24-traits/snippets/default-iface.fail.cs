Dog dog = new Dog();
ISpeaker viaInterface = dog;

Console.WriteLine(viaInterface.Shout()); // works through the interface type
Console.WriteLine(dog.Shout());          // not through the class type

interface ISpeaker
{
    string Speak();

    string Shout() => Speak().ToUpper();
}

class Dog : ISpeaker
{
    public string Speak() => "woof";
}
