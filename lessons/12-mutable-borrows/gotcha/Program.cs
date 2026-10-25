int[] scores = [10, 20, 30];
ref int first = ref scores[0];
ref int last = ref scores[2];
first += 1;
last += 1;
Console.WriteLine($"[{string.Join(", ", scores)}]");
