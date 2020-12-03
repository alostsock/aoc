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

  def traverse(m, slope, pos \\ {0, 0}, count \\ 0)

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

  defp matrix_from_list(list) do
    height = length(list)
    width = length(Enum.at(list, 0))

    matrix =
      for {row, y} <- Enum.with_index(list),
          {square, x} <- Enum.with_index(row),
          into: %{},
          do: {{x, y}, convert_square(square)}

    {width, height, matrix}
  end

  defp convert_square("."), do: 0
  defp convert_square("#"), do: 1
end
