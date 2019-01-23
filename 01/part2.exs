File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Stream.map(&String.to_integer/1)
|> Stream.cycle
|> Enum.reduce_while({0, %MapSet{}}, fn x, {sum, seen} ->
     sum = sum + x
     if MapSet.member?(seen, sum) do
       {:halt, {sum, seen}}
     else
       {:cont, {sum, MapSet.put(seen, sum)}}
     end
   end)
|> elem(0)
|> IO.puts
