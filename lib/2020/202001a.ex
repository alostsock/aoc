defmodule AOC202001a do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&find_pair/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&find_triplet/0)
  end

  defp nums do
    AOC.input(__MODULE__) |> Enum.map(&String.to_integer/1)
  end

  defp find_pair do
    n = nums()

    result =
      for a <- n,
          b <- n,
          a + b == 2020,
          do: a * b

    result |> Enum.at(0)
  end

  defp find_triplet do
    n = nums()

    result =
      for a <- n,
          b <- n,
          c <- n,
          a + b + c == 2020,
          do: a * b * c

    result |> Enum.at(0)
  end
end
