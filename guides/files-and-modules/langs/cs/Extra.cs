static class Extra
{
    public static string Name() => "Extra.cs joined without a declaration";
}

class Holder
{
    int secret = 1;
    public int Peek() => secret;
}
