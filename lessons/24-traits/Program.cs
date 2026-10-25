Introduce(new Dog());
Introduce(new Robot(7));

static void Introduce(ISpeaker s)   // any type that implements ISpeaker
{
    Console.WriteLine($"{s.Speak()} / {s.Shout()}");
}

interface ISpeaker
{
    string Speak();

    string Shout() => Speak().ToUpper();   // default method
}

class Dog : ISpeaker
{
    public string Speak() => "woof";
}

record Robot(uint Id) : ISpeaker
{
    public string Speak() => $"beep {Id}";
}
