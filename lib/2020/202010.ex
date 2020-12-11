defmodule AOC202010 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    %{1 => dist_1, 3 => dist_3} =
      joltages()
      |> joltage_distribution()

    dist_1 * dist_3
  end

  def part2 do
    arrangements =
      joltages()
      |> Enum.reverse()
      |> Stream.with_index()
      |> Stream.map(fn {joltage, i} -> {i, joltage} end)
      |> Enum.into(%{})
      |> arrange()

    arrangements[0]
  end

  defp joltages do
    adapters =
      AOC.input(__MODULE__)
      |> Stream.map(&String.to_integer/1)
      |> Enum.sort()

    device_joltage = List.last(adapters) + 3

    [0 | adapters] ++ [device_joltage]
  end

  defp increment(n, count), do: n + count

  defp joltage_distribution(joltages) do
    acc = {0, %{}}

    {_, dist_map} =
      joltages
      |> Enum.reduce(acc, fn j, {prev_j, map} ->
        key = j - prev_j
        map = Map.update(map, key, 1, &increment(&1, 1))
        {j, map}
      end)

    dist_map
  end

  defp arrange(j, i \\ 0, cache \\ %{}) do
    # Look ahead for possible compatible adapters (within 3 jolts)
    possible_steps =
      (i + 3)..(i + 1)
      |> Stream.reject(&(j[&1] == nil))
      |> Stream.filter(&(j[i] - j[&1] <= 3))
      |> Enum.to_list()

    cond do
      # Root adapter. Only one way to arrange this.
      length(possible_steps) == 0 ->
        Map.put(cache, i, 1)

      # Multiple ways of arranging possible_steps, sum all
      # divide + conquer with a cache
      length(possible_steps) > 0 ->
        possible_steps
        |> Enum.reduce(cache, fn step, cache ->
          if is_map_key(cache, step) do
            count = cache[step]
            Map.update(cache, i, count, &increment(&1, count))
          else
            cache = arrange(j, step, cache)
            count = cache[step]
            Map.update(cache, i, count, &increment(&1, count))
          end
        end)
    end
  end
end
