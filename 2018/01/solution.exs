defmodule AoC2018.Day01 do
  def parse_input() do
    File.stream!("input.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.to_integer/1)
  end

  def part1(input) do
    Enum.sum(input)
  end

  def part2(input) do
    input
    |> Stream.cycle()
    |> Enum.reduce_while({0, MapSet.new([0])}, fn x, {sum, seen} ->
      sum = sum + x

      if MapSet.member?(seen, sum) do
        {:halt, {sum, seen}}
      else
        {:cont, {sum, MapSet.put(seen, sum)}}
      end
    end)
    |> elem(0)
  end
end

input = AoC2018.Day01.parse_input()

input
|> AoC2018.Day01.part1()
|> IO.puts()

input
|> AoC2018.Day01.part2()
|> IO.puts()
