defmodule AoC2018.Day03 do
  def parse_input() do
    rectangles =
      File.stream!("input.txt")
      |> Stream.map(&String.trim/1)
      |> Enum.map(&parse_claim/1)

    {rectangles, count_overlaps(rectangles)}
  end

  def part1({rectangles, overlaps}) do
    Enum.count(overlaps, fn {_, count} -> count > 1 end)
  end

  def part2({rectangles, overlaps}) do
    Enum.reduce_while(rectangles, nil, fn rectangle, _ ->
      if unique?(rectangle, overlaps) do
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

  defp count_overlaps(rectangles) do
    for {_id, x, y, w, h} <- rectangles,
        i <- x..(x + w - 1),
        j <- y..(y + h - 1) do
      {i, j}
    end
    |> Enum.frequencies()
  end

  defp unique?({_id, x, y, w, h}, overlaps) do
    for i <- x..(x + w - 1),
        j <- y..(y + h - 1) do
      {i, j}
    end
    |> Enum.reduce_while(nil, fn coord, _ ->
      if Map.fetch!(overlaps, coord) > 1 do
        {:halt, false}
      else
        {:cont, true}
      end
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
