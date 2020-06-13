defmodule AoC2019.Day01 do
  def parse_input() do
    File.stream!("input.txt")
    |> Stream.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
  end

  def part1(input) do
    input
    |> Stream.map(&fuel/1)
    |> Enum.sum()
  end

  def part2(input) do
    input
    |> Stream.map(&fuel_recursive/1)
    |> Enum.sum()
  end

  defp fuel(mass) do
    div(mass, 3) - 2
  end

  defp fuel_recursive(mass) do
    mass
    |> Stream.iterate(&fuel/1)
    |> Stream.drop(1)
    |> Stream.take_while(&(&1 > 0))
    |> Enum.sum()
  end
end

input = AoC2019.Day01.parse_input()

input
|> AoC2019.Day01.part1()
|> IO.puts()

input
|> AoC2019.Day01.part2()
|> IO.puts()
