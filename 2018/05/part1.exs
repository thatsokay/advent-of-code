defmodule AoC05_1 do
  def react(polymer, reacted \\ "")
  def react("", reacted) do
    String.reverse(reacted)
  end
  def react(<<c::utf8>>, reacted) do
    String.reverse(<<c>> <> reacted)
  end
  def react(<<c::utf8, d::utf8, polymer::binary>>, "") do
    if abs(c - d) === 32 do
      react(polymer, "")
    else
      react(<<d>> <> polymer, <<c>>)
    end
  end
  def react(<<c::utf8, d::utf8, polymer::binary>>, <<e::utf8, reacted::binary>>) do
    if abs(c - d) === 32 do
      react(<<e>> <> polymer, reacted)
    else
      react(<<d>> <> polymer, <<c, e>> <> reacted)
    end
  end
end

File.stream!("input.txt")
|> Enum.at(0)
|> String.trim
|> AoC05_1.react
|> String.length
|> IO.puts
