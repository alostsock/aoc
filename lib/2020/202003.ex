defmodule AOC202003 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    pattern()
    |> traverse({3, 1})
  end

  def part2 do
    p = pattern()
    slopes = [{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}]

    Enum.reduce(slopes, 1, fn slope, acc ->
      acc * traverse(p, slope)
    end)
  end

  def traverse(matrix, slope, pos \\ {0, 0}, tree_count \\ 0)

  def traverse(m, slope, pos, count) do
    {width, height, matrix} = m
    {u, v} = slope
    {x, y} = pos

    if y < height do
      new_count = count + matrix[{rem(x, width), y}]
      traverse(m, slope, {x + u, y + v}, new_count)
    else
      count
    end
  end

  def pattern do
    AOC.input(__MODULE__)
    |> Stream.map(&String.graphemes/1)
    |> Enum.to_list()
    |> matrix_from_list()
  end

  def matrix_from_list(list) do
    height = length(list)
    width = length(Enum.at(list, 0))

    matrix =
      Stream.zip(list, 0..(height - 1))
      |> Stream.flat_map(fn {row, y} ->
        Stream.zip(row, 0..(width - 1))
        |> Stream.map(fn {square, x} ->
          {{x, y}, square}
        end)
      end)
      |> Enum.reduce(Map.new(), fn {pair, square}, acc ->
        case square do
          "." -> Map.put(acc, pair, 0)
          "#" -> Map.put(acc, pair, 1)
        end
      end)

    {width, height, matrix}
  end
end
