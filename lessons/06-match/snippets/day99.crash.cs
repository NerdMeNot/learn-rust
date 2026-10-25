var day = 99;
var kind = day switch
{
    >= 1 and <= 5 => "weekday",
    6 or 7 => "weekend",
};
Console.WriteLine(kind);
