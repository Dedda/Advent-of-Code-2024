// See https://aka.ms/new-console-template for more information

var input = File.ReadAllLines("input.txt");
var parsed = ParseInput(input);

Console.WriteLine($"Part 1: {Part1(parsed)}");
Console.WriteLine($"Part 2: {Part2(parsed)}");

return;

List<List<int>> ParseInput(string[] lines)
{
    return lines
        .Select(line => line
            .Split(' ')
            .Select(int.Parse)
            .ToList())
        .ToList();
}

int Part1(List<List<int>> reports)
{
    return reports.Where(IsSafe).Count();
}

int Part2(List<List<int>> reports)
{
    return reports.Where(report =>
    {
        if (IsSafe(report))
        {
            return true;
        }
        for (int i = 0; i < report.Count; i++)
        {
            var dampened = new List<int>(report);
            dampened.RemoveAt(i);
            if (IsSafe(dampened))
            {
                return true;
            }
        }
        return false;
    }).Count();
}

bool IsSafe(List<int> report)
{
    return AllIncreasing(report) || AllDecreasing(report);
}

bool AllIncreasing(List<int> report)
{
    return report[1..].Aggregate((report[0], true),
        (prod, current) =>
        {
            var valid = prod.Item2 && current > prod.Item1 && current - prod.Item1 <= 3;
            return (current, valid);
        }).Item2;
}

bool AllDecreasing(List<int> report)
{
    return report[1..].Aggregate((report[0], true),
        (prod, current) =>
        {
            var valid = prod.Item2 && current < prod.Item1 && prod.Item1 - current <= 3;
            return (current, valid);
        }).Item2;
}