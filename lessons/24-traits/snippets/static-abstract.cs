Console.WriteLine(Describe<Dog>());

static string Describe<T>() where T : IMaker => T.Make(); // like T::make() in Rust

interface IMaker
{
    static abstract string Make();
}

class Dog : IMaker
{
    public static string Make() => "dog";
}
