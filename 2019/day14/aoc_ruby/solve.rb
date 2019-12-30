input = File.read('../resources/input.txt')
#input = <<-EOS
#157 ORE => 5 NZVS
#165 ORE => 6 DCFZ
#44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
#12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
#179 ORE => 7 PSHF
#177 ORE => 5 HKGWZ
#7 DCFZ, 7 PSHF => 2 XJWVT
#165 ORE => 2 GPVTF
#3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
#EOS
# expect 13312

#input = <<-EOS
#2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
#17 NVRVD, 3 JNWZP => 8 VPVL
#53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
#22 VJHF, 37 MNCFX => 5 FWMGM
#139 ORE => 4 NVRVD
#144 ORE => 7 JNWZP
#5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
#5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
#145 ORE => 6 MNCFX
#1 NVRVD => 8 CXFTF
#1 VJHF, 6 MNCFX => 4 RFSQX
#176 ORE => 6 VJHF
#EOS
# expect 180697

#input = <<-EOS
#171 ORE => 8 CNZTR
#7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
#114 ORE => 4 BHXH
#14 VRPVC => 6 BMBT
#6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
#6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
#15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
#13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
#5 BMBT => 4 WPTQ
#189 ORE => 9 KTJDG
#1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
#12 VRPVC, 27 CNZTR => 2 XDBXC
#15 KTJDG, 12 BHXH => 5 XCVML
#3 BHXH, 2 VRPVC => 7 MZWV
#121 ORE => 7 VRPVC
#7 XCVML => 6 RJRHP
#5 BHXH, 4 VRPVC => 5 LTCX
#EOS
# expect 2210736

hash = {}

input.lines.map do |x|
  y, z = x.strip.split(' => ')
  unit, key = z.split(' ')
  ore = false
  patterns = y.split(', ').map do |xx|
    need_unit, name = xx.split(' ')
    ore = name == 'ORE'
    { name: name, need_unit: need_unit.to_i }
  end
  hash[key] = { unit: unit.to_i, ore: ore, patterns: patterns, used: key == 'FUEL' ? true : false }
end

def contains_only_fuel?(pattern_name, hash_map)
  hash_map
    .filter {|k, v| !v[:used]}
    .all? {|k, v| v[:patterns].none? {|p| p[:name] == pattern_name}}
end

def replace(pattern, hash_map, used_keys, index)
  # puts (" " * index) + "pattern: #{pattern}"
  pat = hash_map[pattern[:name]]
  if pat[:ore]
    # puts (" " * index) + "ore pattern: #{pattern}"
    return [pattern]
  end
  unless contains_only_fuel?(pattern[:name], hash_map)
    # puts (" " * index) + "not contains only fuel: #{pattern}"
    return [pattern]
  end
  used_keys << pattern[:name]
  need_unit = pattern[:need_unit]
  unit = pat[:unit]
  mul = if need_unit % unit == 0
          need_unit / unit
        else
          need_unit / unit + 1
        end
  pat[:patterns].map { |p| pp = p.dup; pp[:need_unit] = p[:need_unit] * mul; pp }
end

ore_keys = hash.filter {|k, v| v[:ore]}.map {|k, _| k}
index = 0
used_keys = []
patterns = hash['FUEL'][:patterns]
loop do
  replaced_array = patterns.map { |p| replace(p, hash, used_keys, index) }.flatten
  # puts (" " * index) + "#{replaced_array}"
  used_keys.each do |k|
    hash[k][:used] = true # mark as used pattern
  end
  patterns = replaced_array.group_by {|i| i[:name]}.map do |k, v|
    { name: k, need_unit: v.reduce(0) {|acc, vv| acc + vv[:need_unit]}}
  end
  # puts (" " * index) + "#{patterns}"
  break if patterns.all? {|p| ore_keys.include?(p[:name])}
  index += 1
end

value = patterns.map do |p|
  pat = hash[p[:name]]
  mul = if p[:need_unit] % pat[:unit] == 0
    p[:need_unit] / pat[:unit]
  else
    p[:need_unit] / pat[:unit] + 1
  end
  pat[:patterns].first[:need_unit] * mul
end.sum

puts "first: #{value}"

