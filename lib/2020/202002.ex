defmodule AOC202002 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    input() |> Stream.filter(&validate1/1) |> Enum.count()
  end

  def part2 do
    input() |> Stream.filter(&validate2/1) |> Enum.count()
  end

  def input do
    AOC.input(__MODULE__)
    |> Stream.map(fn item ->
      Regex.named_captures(~r/(?<a>\d+)-(?<b>\d+) (?<letter>.): (?<pw>.+)/, item)
    end)
    |> Stream.map(fn captures ->
      %{
        captures
        | "a" => String.to_integer(captures["a"]),
          "b" => String.to_integer(captures["b"])
      }
    end)
  end

  def validate1(%{"letter" => letter, "a" => a, "b" => b, "pw" => pw}) do
    count =
      String.graphemes(pw)
      |> Stream.filter(&(&1 == letter))
      |> Enum.count()

    count >= a and count <= b
  end

  def validate2(%{"letter" => letter, "a" => a, "b" => b, "pw" => pw}) do
    letters = String.graphemes(pw)

    pair =
      [Enum.at(letters, a - 1), Enum.at(letters, b - 1)]
      |> Enum.uniq()

    length(pair) == 2 and Enum.member?(pair, letter)
  end
end
