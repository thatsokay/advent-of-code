defmodule AoC04_2 do
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
        slept = Map.update(slept, guard, {%{}, {0, 0}}, &(&1))
        |> Map.update(guard, {%{}, {0, 0}}, fn {minutes, _} ->
          new_minutes = Enum.reduce(start_sleep..end_sleep - 1, minutes, fn minute, sleep ->
            Map.update(sleep, minute, 1, &(&1 + 1))
          end)
          {
            new_minutes,
            Map.to_list(new_minutes)
            |> Enum.max_by(&(elem(&1, 1))),
          }
        end)
        count_sleep(records, slept, guard, start_sleep)
    end
  end
  def count_sleep([], slept, _, _) do
    slept
  end
  def strategy_2(slept) do
    {guard, {_, {minute, _}}} = Map.to_list(slept)
    |> Enum.max_by(&(elem(elem(elem(&1, 1), 1), 1)))
    {
      guard,
      minute,
    }
  end
end

File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Enum.sort
|> AoC04_2.count_sleep
|> AoC04_2.strategy_2
|> (fn {guard, minute} -> guard * minute end).()
|> IO.puts
