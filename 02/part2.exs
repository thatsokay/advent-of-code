defmodule AoC02_2 do
  def difference(x, y) do
    Enum.zip(String.to_charlist(x), String.to_charlist(y))
    |> Enum.count(fn {c, d} ->
      c !== d
    end)
  end
  def find_ids([h | t]) do
    match = Enum.find(t, fn x ->
      AoC02_2.difference(x, h) === 1
    end)
    if match do
      AoC02_2.common(h, match)
    else
      find_ids(t)
    end
  end
  def common(x, y) do
    Enum.zip(String.to_charlist(x), String.to_charlist(y))
    |> Enum.filter(fn {c, d} ->
      c === d
    end)
    |> Enum.unzip
    |> elem(0)
  end
end

File.stream!("input.txt")
|> Enum.map(&String.trim/1)
|> AoC02_2.find_ids
|> IO.puts
