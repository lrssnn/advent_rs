using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Advent;
using FluentAssertions;

namespace AdventTwentyOne;

public class Day1 : Day
{
    public override string DayName => "01";
    public override string Answer1 => "1393";
    public override string Answer2 => "1359";

    private List<int> Data { get; set; }

    public Day1() : base("2021/input1")
    {
        Data = Input.Trim()
            .Split("\n")
            .Select(s => s.Trim())
            .Select(s => int.Parse(s))
            .ToList();
    }

    public override void Solve()
    {
        var last = int.MaxValue;
        var increases = 0;
        foreach (var depth in Data)
        {
            if(depth > last)
            {
                increases++;
            }
            last = depth;
        }
        Result1 = increases.ToString();

        last = int.MaxValue;
        increases = 0;
        for (var i = 2; i < Data.Count; i++)
        {
            var value = Data[i - 2] + Data[i - 1] + Data[i];
            if (value > last) increases++;
            last = value;
        }

        Result2 = increases.ToString();
    }

    public override string ToString() => $"Part 1: {Result1} | Part 2: {Result2}";
}
