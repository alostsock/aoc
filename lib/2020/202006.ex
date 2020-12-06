defmodule AOC202006 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    groups() |> Stream.map(&count_unique_answers/1) |> Enum.sum()
  end

  def part2 do
    groups() |> Stream.map(&count_common_answers/1) |> Enum.sum()
  end

  defp groups do
    AOC.input(__MODULE__)
    |> Stream.chunk_by(&(&1 == ""))
    |> Stream.filter(&(&1 != [""]))
  end

  defp answers(group), do: List.to_string(group) |> String.graphemes()

  defp count_unique_answers(group) do
    answers(group)
    |> Enum.uniq()
    |> Enum.count()
  end

  defp count_common_answers(group) do
    answers(group)
    |> Enum.frequencies()
    |> Map.to_list()
    |> Enum.filter(fn {_, count} -> count == length(group) end)
    |> Enum.count()
  end
end
