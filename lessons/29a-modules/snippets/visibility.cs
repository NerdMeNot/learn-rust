using System.Diagnostics;

class Visibility
{
    static void Write(string root, string path, string text)
    {
        var full = Path.Combine(root, path);
        Directory.CreateDirectory(Path.GetDirectoryName(full)!);
        File.WriteAllText(full, text);
    }

    // Builds the app project (which references the lib project) and prints the
    // compiler's error lines, or "ok" and what the program printed.
    static void Build(string root, string label)
    {
        Console.WriteLine(label);
        var p = Process.Start(new ProcessStartInfo("dotnet", "run --project app -v q -nologo -clp:NoSummary")
        {
            WorkingDirectory = root,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
        })!;
        string output = p.StandardOutput.ReadToEnd() + p.StandardError.ReadToEnd();
        p.WaitForExit();
        var seen = new HashSet<string>();
        foreach (var raw in output.Split('\n'))
        {
            var line = raw.Trim();
            int bracket = line.LastIndexOf(" [");
            if (bracket > 0) line = line[..bracket];
            line = line.Replace(root + "/", "");
            int help = line.IndexOf(" (are you missing");
            if (help > 0) line = line[..help];
            if (line.Contains(": error ") && seen.Add(line))
                Console.WriteLine("  " + line);
            else if (p.ExitCode == 0 && line.Length > 0 && !line.Contains("warning"))
                Console.WriteLine("  " + line);
        }
    }

    static void Main()
    {
        var root = Directory.CreateTempSubdirectory("csvis").FullName;
        Write(root, "lib/lib.csproj", "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>\n");
        Write(root, "app/app.csproj", "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup><ItemGroup><ProjectReference Include=\"../lib/lib.csproj\"/></ItemGroup></Project>\n");
        Write(root, "lib/Lib.cs", @"namespace Lib;

public class Box
{
    public int Open() => 2;
    internal int Inner() => 3;
}

class Plain { }
");
        Write(root, "app/Program.cs", "System.Console.WriteLine(new Lib.Box().Open());\n");
        Build(root, "1. another project calls a public member");
        Write(root, "app/Program.cs", "System.Console.WriteLine(new Lib.Box().Inner());\nvar p = new Lib.Plain();\n");
        Build(root, "2. it calls an internal member, and uses a type with no modifier");
        Write(root, "lib/lib.csproj", "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup><ItemGroup><InternalsVisibleTo Include=\"app\"/></ItemGroup></Project>\n");
        Write(root, "app/Program.cs", "System.Console.WriteLine(new Lib.Box().Inner());\nvar p = new Lib.Plain();\n");
        Build(root, "3. the same code, after the library names app as a friend");
        Write(root, "lib/Lib.cs", @"namespace Lib;

public class Box
{
    int secret() => 1;
}

public class Neighbour
{
    public int Peek() => new Box().secret();
}

public class Outer
{
    public class Nested { int hidden() => 4; }
    public int Peek() => new Nested().hidden();
}
");
        Write(root, "app/Program.cs", "System.Console.WriteLine(1);\n");
        Build(root, "4. a class in the same namespace, and an outer class, read a private member");
    }
}
