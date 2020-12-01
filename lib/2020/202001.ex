defmodule AOC202001 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    solve(2)
  end

  def part2 do
    solve(3)
  end

  defp solve(n) do
    nums()
    |> replicate_stream(n)
    |> Stream.filter(&(Enum.sum(&1) == 2020))
    |> Enum.at(0)
    |> Enum.reduce(fn num, acc -> num * acc end)
  end

  defp nums do
    AOC.input(__MODULE__) |> Enum.map(&String.to_integer/1)
  end

  def replicate_stream(stream, n) when n > 0 do
    stream |> replicate_stream(stream, n, 1)
  end

  defp replicate_stream(acc, stream, n, depth) when depth < n do
    acc
    |> Stream.flat_map(fn elements ->
      stream
      |> Stream.map(&[&1 | List.wrap(elements)])
    end)
    |> replicate_stream(stream, n, depth + 1)
  end

  defp replicate_stream(acc, _, _, _) do
    acc
  end
end
