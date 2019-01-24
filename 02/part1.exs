defmodule AoC02_1 do
  def count(<<>>, counts) do
    counts
  end
  def count(<<c::utf8, rest::binary>>, counts) do
    {_, counts} = Map.get_and_update(counts, c, fn val ->
      if val do
        {val, val + 1}
      else
        {val, 1}
      end
    end)
    count(rest, counts)
  end
  def sum_counts(counts, acc) do
    {_, acc} = Map.get_and_update(acc, 2, fn val ->
      if Enum.member?(Map.values(counts), 2) do
        {val, val + 1}
      else
        {val, val}
      end
    end)
    {_, acc} = Map.get_and_update(acc, 3, fn val ->
      if Enum.member?(Map.values(counts), 3) do
        {val, val + 1}
      else
        {val, val}
      end
    end)
    acc
  end
  def checksum(%{2 => two, 3 => three}) do
    two * three
  end
end

File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Stream.map(&(AoC02_1.count(&1, %{})))
|> Enum.reduce(%{2 => 0, 3 => 0}, &AoC02_1.sum_counts/2)
|> AoC02_1.checksum
|> IO.inspect