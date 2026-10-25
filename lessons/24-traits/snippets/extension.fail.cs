// An extension method adds Speak() to string...
Console.WriteLine("hi".Speak());

ISpeaker s = "hi"; // ...but string is still not an ISpeaker

interface ISpeaker
{
    string Speak();
}

static class StringExtensions
{
    public static string Speak(this string text) => $"the text {text}";
}
