defmodule AOC202009 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    numbers()
    |> validate(25)
  end

  def part2 do
    nums = numbers()
    {target, _} = nums |> validate(25)

    window =
      nums
      |> Stream.with_index()
      |> Stream.map(fn {n, i} -> {i, n} end)
      |> Enum.into(%{})
      |> find_contiguous_window(target)

    Enum.min(window) + Enum.max(window)
  end

  defp numbers do
    AOC.input(__MODULE__)
    |> Stream.map(&String.to_integer/1)
    |> Enum.to_list()
  end

  defp validate(nums, preamble_size) do
    {preamble, rest} = Enum.split(nums, preamble_size)
    preamble = Enum.into(preamble, MapSet.new())

    Stream.zip(nums, rest)
    |> Stream.transform(preamble, fn {head, current}, preamble ->
      valid = validate_window?(preamble, current)
      next_preamble = preamble |> MapSet.delete(head) |> MapSet.put(current)

      {[{current, valid}], next_preamble}
    end)
    |> Stream.filter(fn {_, valid} -> not valid end)
    |> Stream.take(1)
    |> Enum.at(0)
  end

  defp validate_window?(preamble, current) do
    MapSet.to_list(preamble)
    |> Enum.find(fn num -> MapSet.member?(preamble, current - num) end)
    |> case do
      nil -> false
      _ -> true
    end
  end

  defp find_contiguous_window(nums, target) do
    head = 0
    tail = 0
    sum = nums[0]

    find_contiguous_window(nums, head, tail, sum, target)
  end

  defp find_contiguous_window(nums, h, t, s, target) do
    head_sum = s - nums[h]
    tail_sum = s + nums[t + 1]

    cond do
      s == target ->
        Enum.map(h..t, &nums[&1])

      tail_sum <= target ->
        find_contiguous_window(nums, h, t + 1, tail_sum, target)

      head_sum <= target ->
        find_contiguous_window(nums, h + 1, t, head_sum, target)
    end
  end
end
