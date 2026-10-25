var day = 6;
var kind = day switch
{
    >= 1 and <= 5 => "weekday",
    6 or 7 => "weekend",
    _ => "not a day",       // _ means "anything else"
};
Console.WriteLine(kind);
