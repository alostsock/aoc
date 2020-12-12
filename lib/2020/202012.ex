defmodule AOC202012 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  defp ship do
    %{x: 0, y: 0, angle: 0}
  end

  defp ship_with_waypoint do
    %{x: 0, y: 0, wp: %{x: 10, y: 1}}
  end

  defguardp has_waypoint(ship) when is_map_key(ship, :wp)

  def part1 do
    ship = instructions() |> Enum.reduce(ship(), &nav/2)
    dist_manhattan(ship[:x], ship[:y])
  end

  def part2 do
    ship = instructions() |> Enum.reduce(ship_with_waypoint(), &nav/2)
    dist_manhattan(ship[:x], ship[:y])
  end

  defp instructions do
    AOC.input(__MODULE__)
    |> Stream.map(&String.split_at(&1, 1))
    |> Enum.map(fn {action, n} -> {String.to_atom(action), String.to_integer(n)} end)
  end

  defp nav({:L, angle}, ship), do: rotate(ship, angle)
  defp nav({:R, angle}, ship), do: rotate(ship, -1 * angle)
  defp nav({:F, n}, ship), do: forward(ship, n)
  defp nav({dir, n}, ship), do: move({dir, n}, ship)

  defp rotate(ship, angle) when has_waypoint(ship) do
    %{x: prev_x, y: prev_y} = ship[:wp]
    {prev_angle, length} = xy_to_line(prev_x, prev_y)
    {x, y} = line_to_xy(prev_angle + angle, length)

    %{ship | wp: %{x: round(x), y: round(y)}}
  end

  defp rotate(ship, angle), do: %{ship | angle: ship[:angle] + angle}

  defp forward(ship, n) when has_waypoint(ship) do
    %{x: u, y: v} = ship[:wp]
    x = ship[:x] + n * u
    y = ship[:y] + n * v
    %{ship | x: x, y: y}
  end

  defp forward(ship, n) do
    {x, y} = line_to_xy(ship[:angle], n)
    x = ship[:x] + round(x)
    y = ship[:y] + round(y)
    %{ship | x: x, y: y}
  end

  defp move({dir, n}, ship) when has_waypoint(ship),
    do: %{ship | wp: move({dir, n}, ship[:wp])}

  defp move({:N, n}, obj), do: %{obj | y: obj[:y] + n}
  defp move({:S, n}, obj), do: %{obj | y: obj[:y] - n}
  defp move({:E, n}, obj), do: %{obj | x: obj[:x] + n}
  defp move({:W, n}, obj), do: %{obj | x: obj[:x] - n}

  def line_to_xy(angle, length) do
    a = angle * :math.pi() / 180
    y = length * :math.sin(a)
    x = length * :math.cos(a)
    {x, y}
  end

  def xy_to_line(x, y) do
    a = :math.atan2(y, x)
    n = :math.sqrt(:math.pow(x, 2) + :math.pow(y, 2))
    a = a * 180 / :math.pi()
    {a, n}
  end

  def dist_manhattan(x, y), do: abs(x) + abs(y)
end
