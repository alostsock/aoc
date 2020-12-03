defmodule AOC202003a do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    input()
    |> traverse({3, 1})
  end

  def part2 do
    lines = input()
    slopes = [{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}]
    Enum.reduce(slopes, 1, fn slope, acc ->
      count = traverse(lines, slope)
      acc * count
    end)
  end

  def input do
    AOC.input(__MODULE__)
    |> Stream.map(&String.graphemes/1)
  end

  def traverse(stream, {x, y}) do
    stream
    |> Stream.take_every(y)
    |> Stream.map(&Stream.cycle/1)
    |> Stream.transform(0, fn row, shift ->
      Enum.at(row, shift)
      |> case do
        "." -> {[0], shift + x}
        "#" -> {[1], shift + x}
      end
    end)
    |> Enum.sum()
  end
end
