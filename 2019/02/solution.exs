defmodule AoC2019_02 do
  def run_intcode(intcode, {noun, verb}) do
    intcode
    |> put_elem(1, noun)
    |> put_elem(2, verb)
    |> run_ops
  end

  defp run_ops(intcode, ip \\ 0) do
    case elem(intcode, ip) do
      1 ->
        intcode
        |> operate(ip, &Kernel.+/2)
        |> run_ops(ip + 4)
      2 ->
        intcode
        |> operate(ip, &Kernel.*/2)
        |> run_ops(ip + 4)
      99 -> intcode
    end
  end

  defp operate(intcode, ip, op) do
    put_elem(
      intcode,
      elem(intcode, ip + 3),
      op.(
        elem(intcode, elem(intcode, ip + 1)),
        elem(intcode, elem(intcode, ip + 2))
      )
    )
  end
end


input = File.stream!("input.txt")
|> Enum.at(0)
|> String.split(",")
|> Enum.map(&String.to_integer/1)
|> List.to_tuple

input
|> AoC2019_02.run_intcode({12, 2})
|> elem(0)
|> IO.puts

1..99
|> Stream.flat_map(fn(i) -> Stream.map(1..99, fn(j) -> {i, j} end) end)
|> Enum.find(fn(initial) ->
  AoC2019_02.run_intcode(input, initial)
  |> elem(0)
  |> Kernel.===(19690720)
end)
|> (fn({noun, verb}) -> noun * 100 + verb end).()
|> IO.puts