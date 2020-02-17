require 'set'

max = 1000
answer = Set[]

(1..max/4).each{|c|
  (1..(c-1))
    .to_a
    .map {|x| x * (2 * c - x)}
    .combination(2){|a, b|
    if a + b == c**2
      answer << [b / a.to_f, c**2 / a.to_f]
    end
  }
}

pp answer.size
