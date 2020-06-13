defmodule AoC2019.Day02 do
  def parse_input() do
    File.stream!("input.txt")
    |> Enum.at(0)
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end

  def part1(input) do
    run_intcode(input, {12, 2})
  end

  def part2(input) do
    {noun, verb} =
      for noun <- 0..99, verb <- 0..99 do
        {noun, verb}
      end
      |> Enum.find(&(run_intcode(input, &1) === 19_690_720))

    100 * noun + verb
  end

  defp run_intcode(intcode, {noun, verb}) do
    intcode
    |> put_elem(1, noun)
    |> put_elem(2, verb)
    |> run_ops
  end

  defp run_ops(intcode, ip \\ 0) do
    case elem(intcode, ip) do
      1 ->
        intcode
        |> operate(ip, &+/2)
        |> run_ops(ip + 4)

      2 ->
        intcode
        |> operate(ip, &*/2)
        |> run_ops(ip + 4)

      99 ->
        intcode
        |> elem(0)
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

input = AoC2019.Day02.parse_input()

input
|> AoC2019.Day02.part1()
|> IO.puts()

input
|> AoC2019.Day02.part2()
|> IO.puts()
