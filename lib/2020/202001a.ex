defmodule AOC202001a do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&find_pair/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&find_triplet/0)
  end

  defp nums do
    AOC.input(__MODULE__) |> Stream.map(&String.to_integer/1)
  end

  defp find_pair do
    stream =
      for a <- nums(),
          b <- nums(),
          a + b == 2020,
          do: a * b

    stream |> Enum.at(0)
  end

  defp find_triplet do
    stream =
      for a <- nums(),
          b <- nums(),
          c <- nums(),
          a + b + c == 2020,
          do: a * b * c

    stream |> Enum.at(0)
  end
end
