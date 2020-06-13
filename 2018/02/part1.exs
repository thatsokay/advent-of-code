defmodule AoC02_1 do
  def count(<<>>, counts) do
    counts
  end

  def count(<<c::utf8, rest::binary>>, counts) do
    counts =
      Map.update(counts, c, fn val ->
        if val do
          {val, val + 1}
        else
          {val, 1}
        end
      end)

    count(rest, counts)
  end

  def sum_counts(counts, acc) do
    Map.update(acc, 2, 0, fn val ->
      if Enum.member?(Map.values(counts), 2) do
        val + 1
      else
        val
      end
    end)
    |> Map.update(3, 0, fn val ->
      if Enum.member?(Map.values(counts), 3) do
        val + 1
      else
        val
      end
    end)
  end

  def checksum(%{2 => two, 3 => three}) do
    two * three
  end
end

File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Stream.map(&AoC02_1.count(&1, %{}))
|> Enum.reduce(%{2 => 0, 3 => 0}, &AoC02_1.sum_counts/2)
|> AoC02_1.checksum()
|> IO.inspect()
