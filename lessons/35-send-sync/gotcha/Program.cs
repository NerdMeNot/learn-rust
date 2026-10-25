var log = new List<string>();
var job = new Job("resize", log);

var t = new Thread(() => Run(job));
t.Start();
t.Join();
Console.WriteLine("log: [" + string.Join(", ", log.Select(s => $"\"{s}\"")) + "]");

static void Run(Job job) => job.Log.Add(job.Name);

record Job(string Name, List<string> Log);
