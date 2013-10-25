FIZZ = "Fizz"
BUZZ = "Buzz"

func is_fizz(x)
  return x % 3 == 0
end

func is_buzz(x)
  return x % 5 == 0
end

object fizzbuzz

  def apply(x)
    @fizz = is_fizz(x)
    @buzz = is_buzz(x)
    @origin = x
    return @
  end,

  def result
    if @fizz && @buzz
       return FIZZ + BUZZ
    else if @fizz
       return FIZZ
    else if @buzz
       return BUZZ
    else
       return @origin
    end
  end
end


for i in 1..100
    puts fizzbuzz.apply(i).result()
end
