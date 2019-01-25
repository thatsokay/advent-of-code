Code.require_file("part1.exs")

defmodule AoC03_2 do
  def find_unique({id, x, y, w, h}, {fabric, unique}) do
    Enum.reduce(x..x+w-1, {fabric, MapSet.put(unique, id)}, fn x_i, acc ->
      Enum.reduce(y..y+h-1, acc, fn y_i, {fabric, unique} ->
        {val, fabric} = Map.get_and_update(fabric, {x_i, y_i}, fn val ->
          val = if val, do: [id] ++ val, else: [id]
          {val, val}
        end)
        case val do
          [_] ->
            {fabric, unique}
          id_list ->
            {
              fabric,
              Enum.reduce(id_list, unique, fn curr_id, acc ->
                MapSet.delete(acc, curr_id)
              end),
            }
        end
      end)
    end)
  end
end

File.stream!("input.txt")
|> Stream.map(&String.trim/1)
|> Stream.map(&AoC03_1.parse_claim/1)
|> Enum.reduce({%{}, MapSet.new}, &AoC03_2.find_unique/2)
|> elem(1)
|> IO.inspect(char_lists: :as_lists)
