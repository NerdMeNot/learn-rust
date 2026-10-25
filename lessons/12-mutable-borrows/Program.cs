List<int> scores = [10, 20];
AddBonus(scores, 5);
Console.WriteLine($"[{string.Join(", ", scores)}]");

static void AddBonus(List<int> scores, int bonus)
{
    for (var i = 0; i < scores.Count; i++)
    {
        scores[i] += bonus;
    }
    scores.Add(0);
}
