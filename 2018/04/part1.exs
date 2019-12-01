defmodule AoC04_1 do
  def count_sleep(records, slept \\ %{}, guard \\ 0, start_sleep \\ 0)
  def count_sleep([record | records], slept, guard, start_sleep) do
    parts = String.split(record, [" ", "]", ":", "#"])
    cond do
      String.contains?(record, "#") ->
        count_sleep(records, slept, String.to_integer(Enum.at(parts, 6)), start_sleep)
      String.contains?(record, "asleep") ->
        count_sleep(records, slept, guard, String.to_integer(Enum.at(parts, 2)))
      String.contains?(record, "up") ->
        end_sleep = String.to_integer(Enum.at(parts, 2))
        slept = Map.update(slept, guard, {%{}, 0}, &(&1))
        |> Map.update(guard, {%{}, 0}, fn {minutes, total} ->
          {
            Enum.reduce(start_sleep..end_sleep - 1, minutes, fn minute, sleep ->
              Map.update(sleep, minute, 1, &(&1 + 1))
            end),
            total + end_sleep - start_sleep,
          }
        end)
        count_sleep(records, slept, guard, start_sleep)
    end
  end
  def count_sleep([], slept, _, _) do
    slept
  end
  def strategy_1(slept) do
    {guard, {minutes, _}} = Map.to_list(slept)
    |> Enum.max_by(&(elem(elem(&1, 1), 1)))
    {
      guard,
      Map.to_list(minutes)
      |> Enum.max_by(&(elem(&1, 1)))
      |> elem(0),
    }
  end
end

File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Enum.sort
|> AoC04_1.count_sleep
|> AoC04_1.strategy_1
|> (fn {guard, minute} -> guard * minute end).()
|> IO.puts
