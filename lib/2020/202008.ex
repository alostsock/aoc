defmodule AOC202008 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    instructions() |> execute()
  end

  def part2 do
    ops = instructions()

    get_debug_points(ops)
    |> Stream.map(fn debug_pt ->
      {op_num, _, _} = debug_pt
      execute(swap_op_at(ops, op_num), debug_pt)
    end)
    |> Stream.filter(fn {stop_reason, _} -> stop_reason == :term end)
    |> Stream.take(1)
    |> Enum.at(0)
  end

  defp instructions do
    AOC.input(__MODULE__)
    |> Stream.with_index()
    |> Stream.map(fn {instruction, i} ->
      [op, n] = String.split(instruction, " ")
      {i, {op, String.to_integer(n)}}
    end)
    |> Enum.into(%{})
  end

  defp execute(ops, state \\ {0, MapSet.new(), 0})

  defp execute(ops, {op_num, visited, acc} = state) when is_map_key(ops, op_num) do
    if MapSet.member?(visited, op_num) do
      {:halt, acc}
    else
      {op, value} = Map.get(ops, op_num)
      execute(ops, next_state(op, value, state))
    end
  end

  defp execute(_, {_, _, acc}), do: {:term, acc}

  defp get_debug_points(ops, state \\ {0, MapSet.new(), 0}, debug_pts \\ [])

  defp get_debug_points(ops, {op_num, visited, acc} = state, debug_pts) when is_map_key(ops, op_num) do
    if MapSet.member?(visited, op_num) do
      debug_pts
    else
      {op, value} = Map.get(ops, op_num)

      debug_pts =
        if op == "nop" or op == "jmp",
          do: [{op_num, visited, acc} | debug_pts],
          else: debug_pts

      get_debug_points(ops, next_state(op, value, state), debug_pts)
    end
  end

  defp get_debug_points(_, {_, _, acc}, _), do: {:term, acc}

  defp next_state(op, value, state) do
    {op_num, visited, acc} = state
    visited = MapSet.put(visited, op_num)

    case op do
      "nop" -> {op_num + 1, visited, acc}
      "acc" -> {op_num + 1, visited, acc + value}
      "jmp" -> {op_num + value, visited, acc}
    end
  end

  defp swap_op_at(ops, op_num) do
    {op, value} = Map.get(ops, op_num)

    case op do
      "nop" -> %{ops | op_num => {"jmp", value}}
      "jmp" -> %{ops | op_num => {"nop", value}}
      _ -> ops
    end
  end
end
