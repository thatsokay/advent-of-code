defmodule AoC2019.Day03 do
  def parse_input() do
    File.stream!("input.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&parse_line/1)
    |> Enum.map(&trace_wire/1)
    |> List.to_tuple()
  end

  def part1(input) do
    {wire1, wire2} = input

    intersections(wire1, wire2)
    |> Stream.map(fn {x, y} -> abs(x) + abs(y) end)
    |> Enum.min()
  end

  def part2(input) do
    {wire1, wire2} = input

    intersections(wire1, wire2)
    |> Stream.map(fn location -> wire1[location] + wire2[location] end)
    |> Enum.min()
  end

  defp parse_line(line) do
    line
    |> String.split(",")
    |> Enum.map(&parse_move/1)
  end

  defp parse_move(<<direction, rest::binary>>) do
    direction =
      case direction do
        ?U -> {0, -1}
        ?D -> {0, 1}
        ?L -> {-1, 0}
        ?R -> {1, 0}
      end

    {direction, String.to_integer(rest)}
  end

  defp trace_wire(wire) do
    {visited, _, _} = Enum.reduce(wire, {%{}, {0, 0}, 0}, &wire_move/2)
    visited
  end

  defp wire_move({{dir_x, dir_y}, distance}, acc) do
    1..distance
    |> Enum.reduce(acc, fn _, {visited, {pos_x, pos_y}, steps} ->
      position = {dir_x + pos_x, dir_y + pos_y}
      new_steps = steps + 1
      {Map.put_new(visited, position, new_steps), position, new_steps}
    end)
  end

  defp intersections(wire1, wire2) do
    keys1 = Map.keys(wire1) |> MapSet.new()
    keys2 = Map.keys(wire2) |> MapSet.new()
    MapSet.intersection(keys1, keys2)
  end
end

input = AoC2019.Day03.parse_input()

input
|> AoC2019.Day03.part1()
|> IO.puts()

input
|> AoC2019.Day03.part2()
|> IO.puts()
