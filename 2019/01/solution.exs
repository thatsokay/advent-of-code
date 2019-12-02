defmodule AoC2019_01 do
  def fuel(mass, acc \\ 0) do
    if mass <= 6 do
      acc
    else
      new_mass = div(mass, 3) - 2
      fuel(new_mass, acc + new_mass)
    end
  end
end


input = File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Enum.map(&String.to_integer/1)

input
|> Stream.map(&(div(&1, 3) - 2))
|> Enum.sum
|> IO.puts

input
|> Stream.map(&(AoC2019_01.fuel(&1)))
|> Enum.sum
|> IO.puts
