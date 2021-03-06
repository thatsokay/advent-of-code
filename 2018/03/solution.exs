defmodule AoC2018.Day03 do
  def parse_input() do
    rectangles =
      File.stream!("input.txt")
      |> Stream.map(&String.trim/1)
      |> Enum.map(&parse_claim/1)

    {rectangles, count_layers(rectangles)}
  end

  def part1({_rectangles, layers}) do
    Enum.count(layers, fn {_, count} -> count > 1 end)
  end

  def part2({rectangles, layers}) do
    Enum.find_value(rectangles, fn rectangle ->
      if unique?(rectangle, layers) do
        elem(rectangle, 0)
      end
    end)
  end

  defp parse_claim(claim) do
    [_, id, _, x, y, _, w, h] = String.split(claim, [" ", "#", ",", ":", "x"])

    [id, x, y, w, h]
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end

  defp count_layers(rectangles) do
    for {_id, x, y, w, h} <- rectangles,
        i <- x..(x + w - 1),
        j <- y..(y + h - 1) do
      {i, j}
    end
    |> Enum.frequencies()
  end

  defp unique?({_id, x, y, w, h}, layers) do
    for i <- x..(x + w - 1),
        j <- y..(y + h - 1) do
      {i, j}
    end
    |> Enum.find_value(fn coord -> Map.fetch!(layers, coord) > 1 end)
    |> Kernel.!()
  end
end

input = AoC2018.Day03.parse_input()

input
|> AoC2018.Day03.part1()
|> IO.puts()

input
|> AoC2018.Day03.part2()
|> IO.puts()
