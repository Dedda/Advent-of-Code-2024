// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

var input = File.ReadAllText("input.txt");
var tuples = (from line in input.Split('\n')
    where !string.IsNullOrWhiteSpace(line)
    select ToIntTuple(line)).ToList();

Console.WriteLine($"Part 1: {Part1(tuples)}");
Console.WriteLine($"Part 2: {Part2(tuples)}");
return;

(int, int) ToIntTuple(string line)
{
    var ints = (from part in line.Split(' ')
        where !string.IsNullOrWhiteSpace(part)
        select int.Parse(part)).ToList();
    return (ints[0], ints[1]);
}

int Part1(List<(int, int)> items)
{
    var left = from tuple in items
        orderby tuple.Item1
        select tuple.Item1;
    var right = from tuple in items
        orderby tuple.Item2
        select tuple.Item2;
    return left.Zip(right).Select(row => Math.Abs(row.First - row.Second)).Sum();
}

int Part2(List<(int, int)> items)
{
    var left = from tuple in items select tuple.Item1;
    var right = from tuple in items select tuple.Item2;
    return left.Select(l => l * right.Count(r => r == l)).Sum();
}