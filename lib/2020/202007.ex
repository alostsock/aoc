defmodule AOC202007 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    bags()
    |> create_inner_bag_map()
    |> traverse_inner("shiny gold")
    |> MapSet.size()
  end

  def part2 do
    count =
      bags()
      |> create_outer_bag_map()
      |> traverse_outer("shiny gold")

    # Don't count the shiny gold bag
    count - 1
  end

  defp bags do
    AOC.input(__MODULE__)
    |> Stream.filter(fn rule -> not Regex.match?(~r/no other bags/, rule) end)
    |> Stream.map(fn rule ->
      [_, outer_bag, inner_bags] = Regex.run(~r/(.+) bags contain (.+)\./, rule)

      inner_bags =
        Regex.scan(~r/(\d+) (.+?) bag/, inner_bags, capture: :all_but_first)
        |> Enum.map(fn [num, bag] -> {String.to_integer(num), bag} end)

      {outer_bag, inner_bags}
    end)
  end

  defp create_inner_bag_map(bags) do
    Stream.flat_map(bags, fn {outer_bag, inner_bags} ->
      Enum.map(inner_bags, fn {_, inner_bag} -> {inner_bag, outer_bag} end)
    end)
    |> Enum.reduce(%{}, fn {inner_bag, outer_bag}, map ->
      default = List.wrap(outer_bag)
      Map.update(map, inner_bag, default, fn prev -> [outer_bag | prev] end)
    end)
  end

  defp traverse_inner(map, key, acc \\ MapSet.new()) do
    if is_map_key(map, key) do
      outer_bags = map[key]

      Enum.reduce(outer_bags, acc, fn bag, acc ->
        traverse_inner(map, bag, MapSet.put(acc, bag))
      end)
    else
      acc
    end
  end

  defp create_outer_bag_map(bags), do: bags |> Enum.into(%{})

  defp traverse_outer(map, key, acc \\ 0) do
    if is_map_key(map, key) do
      inner_bags = map[key]

      inner_count =
        Enum.reduce(inner_bags, 0, fn {num, bag}, acc ->
          acc + num * traverse_outer(map, bag)
        end)

      # Account for the outer bag
      acc + inner_count + 1
    else
      1
    end
  end
end
