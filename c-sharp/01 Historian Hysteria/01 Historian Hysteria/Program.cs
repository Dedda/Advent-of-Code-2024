// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

var input = File.ReadAllText("input.txt");
var tuples = input
    .Split('\n')
    .ToList()
    .Where(x => !string.IsNullOrWhiteSpace(x))
    .Select(line => line
        .Split(" ")
        .Where(item => !string.IsNullOrWhiteSpace(item))
        .Select(int.Parse)
        .ToList())
    .Select(items => (items[0], items[1]))
    .ToList();

Console.WriteLine($"Part 1: {Part1(tuples)}");
Console.WriteLine($"Part 2: {Part2(tuples)}");
return;

int Part1(List<(int, int)> items)
{
    var left = items.Select(tuple => tuple.Item1).ToList();
    var right = items.Select(tuple => tuple.Item2).ToList();
    left.Sort();
    right.Sort();
    return left.Zip(right).Select(row => Math.Abs(row.First - row.Second)).Sum();
}

int Part2(List<(int, int)> items)
{
    var left = items.Select(tuple => tuple.Item1).ToList();
    var right = items.Select(tuple => tuple.Item2).ToList();
    return left.Select(l => l * right.Count(r => r == l)).Sum();
}