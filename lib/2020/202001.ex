defmodule AOC202001 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    find_pair() |> multiply()
  end

  def part2 do
    find_triplet() |> multiply()
  end

  defp multiply(nums) do
    nums |> Enum.reduce(fn n, acc -> n * acc end)
  end

  defp nums do
    AOC.input(__MODULE__) |> Stream.map(&String.to_integer/1)
  end

  defp find_pair do
    nums()
    |> Stream.flat_map(fn a ->
      nums()
      |> Stream.filter(&(a + &1 == 2020))
      |> Stream.map(&[a, &1])
    end)
    |> Enum.at(0)
  end

  defp find_triplet do
    nums()
    |> Stream.flat_map(fn a ->
      nums()
      |> Stream.flat_map(fn b ->
        nums()
        |> Stream.filter(&(a + b + &1 == 2020))
        |> Stream.map(&[a, b, &1])
      end)
    end)
    |> Enum.at(0)
  end
end
