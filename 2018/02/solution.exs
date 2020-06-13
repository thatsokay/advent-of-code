defmodule AoC2018.Day02 do
  def parse_input() do
    File.stream!("input.txt")
    |> Enum.map(&String.trim/1)
  end

  def part1(input) do
    {twos_count, threes_count} =
      input
      |> Stream.map(&has_2_and_3_of_kind/1)
      |> Enum.reduce({0, 0}, fn current, {twos, threes} ->
        case current do
          {true, true} -> {twos + 1, threes + 1}
          {true, false} -> {twos + 1, threes}
          {false, true} -> {twos, threes + 1}
          {false, false} -> {twos, threes}
        end
      end)

    twos_count * threes_count
  end

  def part2([h | t]) do
    match_length = String.length(h) - 1

    match =
      Enum.find_value(t, fn id ->
        common = common_chars(id, h)
        if Enum.count(common) === match_length, do: common
      end)

    if match do
      match
    else
      part2(t)
    end
  end

  defp has_2_and_3_of_kind(id) do
    vals =
      id
      |> count_chars()
      |> Map.values()

    {Enum.member?(vals, 2), Enum.member?(vals, 3)}
  end

  defp count_chars(id) do
    id
    |> String.codepoints()
    |> Enum.reduce(%{}, fn char, acc ->
      Map.update(acc, char, 1, &(&1 + 1))
    end)
  end

  defp common_chars(id1, id2) do
    Enum.zip(String.codepoints(id1), String.codepoints(id2))
    |> Enum.flat_map(fn {c, d} ->
      if c === d, do: [c], else: []
    end)
  end
end

input = AoC2018.Day02.parse_input()

input
|> AoC2018.Day02.part1()
|> IO.puts()

input
|> AoC2018.Day02.part2()
|> IO.puts()
