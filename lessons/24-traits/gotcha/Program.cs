Introduce(new Robot(7));

static void Introduce(ISpeaker s)
{
    Console.WriteLine(s.Speak());
}

interface ISpeaker
{
    string Speak();
}

record Robot(uint Id)   // has Speak, but never says ": ISpeaker"
{
    public string Speak() => $"beep {Id}";
}
