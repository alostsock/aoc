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
            do: {{x, y}, update_seat({x, y}, map, strategy)}

      populate({new_map, width, height}, strategy)
    end
  end

  defp update_seat(pos, seatmap, :part1) do
    {state, _} = seatmap[pos]

    cond do
      state == :empty and count_adjacent(pos, seatmap) == 0 -> {:occupied, :fresh}
      state == :occupied and count_adjacent(pos, seatmap) >= 4 -> {:empty, :fresh}
      true -> {state, :stale}
    end
  end

  defp update_seat(pos, seatmap, :part2) do
    {state, _} = seatmap[pos]

    cond do
      state == :empty and count_linear(pos, seatmap) == 0 -> {:occupied, :fresh}
      state == :occupied and count_linear(pos, seatmap) >= 5 -> {:empty, :fresh}
      true -> {state, :stale}
    end
  end

  defp count_adjacent(pos, seatmap) do
    adjacent_coords(pos)
    |> Enum.map(fn coord -> seatmap[coord] end)
    |> count_occupied_seats()
  end

  defp count_linear(pos, seatmap) do
    directions()
    |> Enum.map(fn dir -> look(pos, seatmap, dir) end)
    |> count_occupied_seats()
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
    for u <- (x - 1)..(x + 1),
        v <- (y - 1)..(y + 1),
        {u, v} != {x, y},
        do: {u, v}
  end

  defp directions() do
    for u <- -1..1,
        v <- -1..1,
        {u, v} != {0, 0},
        do: {u, v}
  end

  defp look({x, y}, seatmap, {u, v} = dir) do
    pos = {x + u, y + v}
    seat = seatmap[pos]

    case seat do
      {:floor, _} -> look(pos, seatmap, dir)
      _ -> seat
    end
  end
end
