Console.WriteLine(new Dog().Shout());

interface ISpeaker
{
    string Speak();
}

class Dog : ISpeaker
{
    public string Speak() => "woof";
}

static class SpeakerExtensions
{
    // An extension with a bound: every ISpeaker gets Shout().
    public static string Shout<T>(this T s) where T : ISpeaker => s.Speak().ToUpper();
}
