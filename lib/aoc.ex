defmodule AOC do
  defp input_file(module) do
    %{"year" => year, "day" => day} =
      Regex.named_captures(~r/.*(?<year>\d{4})(?<day>\d{2})/, to_string(module))

    Application.app_dir(:aoc, ["priv", year, day <> ".txt"])
  end

  def input(module) do
    input_file(module)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
  end

  def inspect_time(fun) do
    {time, result} = :timer.tc(fun)
    IO.puts("#{inspect result}\nDone in #{time / 1000} ms.")
  end
end
