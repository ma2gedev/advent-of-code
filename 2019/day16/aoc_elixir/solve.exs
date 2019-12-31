input = File.read!("../resources/input.txt") |> String.trim
input_numbers = input |> String.graphemes |> Enum.map(&String.to_integer/1)

defmodule Solver do
  @phases 100
  @base_pattern [0, 1, 0, -1] 

  def solve(input_numbers) do
    length = input_numbers |> Enum.count
    patterns = create_patterns(length)
    (1..@phases) |> Enum.reduce(input_numbers, fn _, filtered_input ->
      [_, next_input] = patterns
        |> Enum.reduce([filtered_input, []], fn pattern, [input_numbers, next_input] = acc ->
          result = Enum.zip(input_numbers, pattern)
            |> Enum.map(fn {num, p} -> rem(num * p, 10) end)
            |> Enum.sum
          [input_numbers, next_input ++ [abs(rem(result, 10))]]
        end)
      next_input
    end)
  end

  def create_patterns(length) do
    (1..length)
    |> Enum.map(fn count ->
      @base_pattern
      |> Enum.flat_map(fn p ->
        Stream.cycle([p]) |> Enum.take(count)
      end)
      |> Stream.cycle
      |> Enum.take(length + 1)
      |> Enum.drop(1)
    end)
  end
end

first_result = Solver.solve(input_numbers)

IO.puts "first: #{first_result |> Enum.take(8) |> Enum.join}"


