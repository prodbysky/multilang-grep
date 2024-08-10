string file_name = Environment.GetCommandLineArgs()[1];
string needle = Environment.GetCommandLineArgs()[2];

StreamReader file = new StreamReader(file_name);

string? line = file.ReadLine();

while (line != null)
{
    if (line.Contains(needle))
    {
        Console.WriteLine(line);
    }
    line = file.ReadLine();
}
