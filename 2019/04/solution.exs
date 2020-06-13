defmodule AoC2019.Day04 do
  def parse_input() do
    {start, finish} =
      File.stream!("input.txt")
      |> Enum.at(0)
      |> String.split("-")
      |> Enum.map(&String.to_integer/1)
      |> List.to_tuple()

    start..finish
    |> Enum.map(&digits/1)
  end

  def part1(input) do
    input
    |> Stream.filter(&increasing?/1)
    |> Stream.filter(&double?/1)
    |> Enum.count()
  end

  def part2(input) do
    input
    |> Stream.filter(&increasing?/1)
    |> Stream.filter(&double_strict?/1)
    |> Enum.count()
  end

  defp digits(guess) do
    guess
    |> to_charlist()
    |> Enum.map(&(&1 - ?0))
  end

  defp increasing?(digits) do
    digits
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.all?(fn [a, b] -> a <= b end)
  end

  defp double?(digits) do
    digits
    |> Stream.chunk_by(& &1)
    |> Enum.any?(&(Enum.count(&1) >= 2))
  end

  defp double_strict?(digits) do
    digits
    |> Stream.chunk_by(& &1)
    |> Enum.any?(&(Enum.count(&1) === 2))
  end
end

input = AoC2019.Day04.parse_input()

input
|> AoC2019.Day04.part1()
|> IO.puts()

input
|> AoC2019.Day04.part2()
|> IO.puts()
