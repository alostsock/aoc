defmodule AOC202011 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    {map, _, _} = seatmap() |> populate(:part1)

    Map.values(map) |> count_occupied_seats()
  end

  def part2 do
    {map, _, _} = seatmap() |> populate(:part2)

    Map.values(map) |> count_occupied_seats()
  end


  defp seatmap do
    raw = AOC.input(__MODULE__) |> Enum.map(&String.graphemes/1)
    width = length(raw)
    height = length(Enum.at(raw, 0))

    map =
      for {line, x} <- Enum.with_index(raw),
          {state, y} <- Enum.with_index(line),
          into: %{},
          do:
            {{x, y},
             case state do
               "#" -> {:occupied, :fresh}
               "L" -> {:empty, :fresh}
               "." -> {:floor, :fresh}
             end}

    {map, width, height}
  end

  defp populate({map, width, height} = seatmap, strategy) do
    settled =
      Map.values(map)
      |> Enum.all?(fn {_, age} -> age == :stale end)

    if settled do
      seatmap
    else
      new_map =
        for x <- 0..(width - 1),
            y <- 0..(height - 1),
            into: %{},
            do:
              {{x, y},
               case strategy do
                 :part1 -> change_state1({x, y}, map)
                 :part2 -> change_state2({x, y}, map)
               end}

      populate({new_map, width, height}, strategy)
    end
  end

  defp count_occupied_seats(seats) do
    Enum.reduce(seats, 0, fn seat, acc ->
      case seat do
        {:occupied, _} -> acc + 1
        _ -> acc
      end
    end)
  end

  defp adjacent_coords({x, y}) do
    coords = for u <- (x - 1)..(x + 1), v <- (y - 1)..(y + 1), do: {u, v}
    List.delete(coords, {x, y})
  end

  defp count_adjacent_occupied(pos, seatmap) do
    adjacent_coords(pos)
    |> Enum.map(&(seatmap[&1]))
    |> count_occupied_seats()
  end

  defp change_state1(pos, seatmap) do
    {state, _} = seatmap[pos]
    occupied = count_adjacent_occupied(pos, seatmap)

    cond do
      state == :empty and occupied == 0 -> {:occupied, :fresh}
      state == :occupied and occupied >= 4 -> {:empty, :fresh}
      true -> {state, :stale}
    end
  end

  defp directions() do
    directions = for u <- -1..1, v <- -1..1, do: {u, v}
    List.delete(directions, {0, 0})
  end

  defp look({x, y}, seatmap, {u, v} = dir) do
    next_pos = {x + u, y + v}
    next_seat = seatmap[next_pos]
    case next_seat do
      {:floor, _} -> look(next_pos, seatmap, dir)
      _ -> next_seat
    end
  end

  defp count_linear_occupied(pos, seatmap) do
    directions()
    |> Enum.map(&look(pos, seatmap, &1))
    |> count_occupied_seats()
  end

  defp change_state2(pos, seatmap) do
    {state, _} = seatmap[pos]
    count = count_linear_occupied(pos, seatmap)

    cond do
      state == :empty and count == 0 -> {:occupied, :fresh}
      state == :occupied and count >= 5 -> {:empty, :fresh}
      true -> {state, :stale}
    end
  end
end
