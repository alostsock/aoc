defmodule AOC202005 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    boarding_passes()
    |> Stream.map(&get_seat/1)
    |> Stream.map(&get_id/1)
    |> Enum.max()
  end

  def part2 do
    ids =
      boarding_passes()
      |> Stream.map(&get_seat/1)
      |> Stream.map(&get_id/1)
      |> Enum.sort()

    offset = List.first(ids)

    {id_after_empty, _} =
      Stream.with_index(ids)
      |> Stream.filter(fn {id, n} -> id - n != offset end)
      |> Enum.at(0)

    id_after_empty - 1
  end

  defp boarding_passes do
    AOC.input(__MODULE__) |> Stream.map(&String.split_at(&1, -3))
  end

  defp get_id({row, col}), do: row * 8 + col

  defp get_seat(boarding_pass) do
    {row_part, col_part} = boarding_pass

    legend = %{"F" => :low, "B" => :high, "L" => :low, "R" => :high}

    row =
      String.graphemes(row_part)
      |> Enum.map(&Map.fetch!(legend, &1))
      |> Enum.reduce({0, 127}, &partition/2)

    col =
      String.graphemes(col_part)
      |> Enum.map(&Map.fetch!(legend, &1))
      |> Enum.reduce({0, 7}, &partition/2)

    {row, col}
  end

  defp partition(:low, {lower, upper}) when upper - lower == 1, do: lower
  defp partition(:high, {lower, upper}) when upper - lower == 1, do: upper

  defp partition(:low, {lower, upper}) do
    diff = div(upper - lower, 2)
    {lower, lower + diff}
  end

  defp partition(:high, {lower, upper}) do
    diff = div(upper - lower, 2)
    {lower + diff + 1, upper}
  end
end
