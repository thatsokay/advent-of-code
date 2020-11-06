defmodule AoC2018.Day03 do
  def parse_input() do
    File.stream!("input.txt")
    |> Stream.map(&String.trim/1)
    |> Enum.map(&parse_claim/1)
  end

  def part1(input) do
    input
    |> overlaps()
    |> Enum.count(fn {_, count} -> count > 1 end)
  end

  def part2(input) do
    overlap_map = overlaps(input)

    Enum.reduce_while(input, nil, fn rectangle, _ ->
      if unique?(rectangle, overlap_map) do
        {:halt, elem(rectangle, 0)}
      else
        {:cont, nil}
      end
    end)
  end

  defp parse_claim(claim) do
    [_, id, _, x, y, _, w, h] = String.split(claim, [" ", "#", ",", ":", "x"])

    [id, x, y, w, h]
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end

  defp overlaps(rectangles) do
    Enum.reduce(rectangles, %{}, fn {_id, x, y, w, h}, acc ->
      Enum.reduce(x..(x + w - 1), acc, fn i, acc_x ->
        Enum.reduce(y..(y + h - 1), acc_x, fn j, acc_y ->
          Map.update(acc_y, {i, j}, 1, &(&1 + 1))
        end)
      end)
    end)
  end

  defp unique?({_id, x, y, w, h}, overlaps) do
    Enum.reduce_while(x..(x + w - 1), nil, fn i, _ ->
      Enum.reduce_while(y..(y + h - 1), nil, fn j, _ ->
        if Map.fetch!(overlaps, {i, j}) > 1 do
          {:halt, {:halt, false}}
        else
          {:cont, {:cont, true}}
        end
      end)
    end)
  end
end

input = AoC2018.Day03.parse_input()

input
|> AoC2018.Day03.part1()
|> IO.puts()

input
|> AoC2018.Day03.part2()
|> IO.puts()
