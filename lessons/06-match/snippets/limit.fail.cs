int limit = 10;
int n = 10;
string text = n switch
{
    limit => "at the limit",
    _ => "under",
};
Console.WriteLine(text);
