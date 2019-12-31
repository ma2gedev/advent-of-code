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
        |> Enum.reduce([filtered_input, []], fn pattern, [input_numbers, next_input] ->
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


# second
defmodule Solver2 do
  # offset is 5,970,927, number of digits is 6,500,000. so at that region fft consider less thing
  def resolve([], _last_result) do
    []
  end
  def resolve([h | tail], last_result) do
    res = rem(h + last_result, 10)
    [res] ++ resolve(tail, res)
  end
end

offset = input |> String.slice(0..6) |> String.to_integer
repeated_count = 10_000
second_input = Stream.cycle(input_numbers) |> Enum.take(repeated_count * Enum.count(input_numbers))
necessary_values = second_input |> Enum.slice(offset..-1)
second_result = (1..100) |> Enum.reduce(necessary_values, fn _, filtered_input ->
  Solver2.resolve(Enum.reverse(filtered_input), 0) |> Enum.reverse
end)
IO.puts "second: #{second_result |> Enum.take(8) |> Enum.join}"

