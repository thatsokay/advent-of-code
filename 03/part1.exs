defmodule AoC03_1 do
  def parse_claim(claim) do
    [_, id, _, x, y, _, w, h] = String.split(claim, [" ", "#", ",", ":", "x"])
    [id, x, y, w, h]
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple
  end
  def gen_map({_, x, y, w, h}, acc) do
    Enum.reduce(x..x+w-1, acc, fn x_i, acc_x ->
      Enum.reduce(y..y+h-1, acc_x, fn y_i, acc_y ->
        Map.update(acc_y, {x_i, y_i}, 1, &(&1 + 1))
      end)
    end)
  end
end

File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Stream.map(&AoC03_1.parse_claim/1)
|> Enum.reduce(%{}, &AoC03_1.gen_map/2)
|> Enum.count(fn {_, i} -> i > 1 end)
|> IO.puts
