defmodule AOC202004 do
  def run do
    IO.puts("\nRunning p1...")
    AOC.inspect_time(&part1/0)
    IO.puts("\nRunning p2...")
    AOC.inspect_time(&part2/0)
  end

  def part1 do
    passports()
    |> Stream.filter(&required_fields?/1)
    |> Enum.count()
  end

  def part2 do
    passports()
    |> Stream.filter(&required_fields?/1)
    |> Stream.filter(&validate_fields?/1)
    |> Enum.count()
  end

  defp passports do
    AOC.input(__MODULE__)
    |> Stream.chunk_by(&(String.length(&1) > 0))
    |> Stream.map(&Enum.join(&1, " "))
    |> Stream.map(&String.split/1)
    |> Stream.filter(&(length(&1) > 0))
  end

  defp required_fields?(fields) do
    required = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]
    fields = Enum.filter(fields, &(not String.starts_with?(&1, "cid")))

    if length(fields) == length(required) do
      Enum.sort(fields)
      |> Enum.zip(required)
      |> Enum.all?(fn {field, req} ->
        String.starts_with?(field, req)
      end)
    else
      false
    end
  end

  defp validate_fields?(fields) do
    Enum.map(fields, &String.split(&1, ":"))
    |> Enum.all?(fn [key, val] ->
      case key do
        "byr" -> validate_number?(val, 1920, 2002)
        "iyr" -> validate_number?(val, 2010, 2020)
        "eyr" -> validate_number?(val, 2020, 2030)
        "hgt" -> validate_height?(val)
        "hcl" -> Regex.match?(~r/^#\w{6}$/, val)
        "ecl" -> Enum.member?(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"], val)
        "pid" -> Regex.match?(~r/^\d{9}$/, val)
        "cid" -> true
      end
    end)
  end

  defp validate_number?(val, min, max) do
    num = String.to_integer(val)
    num >= min and num <= max
  end

  defp validate_height?(val) do
    {len, unit} = String.split_at(val, -2)

    case unit do
      "cm" -> validate_number?(len, 150, 193)
      "in" -> validate_number?(len, 59, 76)
      _ -> false
    end
  end
end
