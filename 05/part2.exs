Code.require_file("part1.exs")

defmodule AoC05_2 do
  def improve(polymer) do
    Stream.map(?a..?z, fn c ->
      String.replace(polymer, ~r/#{<<c>>}/i, "", global: true)
      |> AoC05_1.react
      |> String.length
    end)
    |> Enum.min
  end
end

File.stream!("input.txt")
|> Enum.at(0)
|> String.trim
|> AoC05_2.improve
|> IO.puts
