const fizzbuzz_strings =
    * "Fizz"
    * "Buzz"
    * "FizzBuzz"

let is_fizzbuzz(x)
    x % 15 == 0

let is_fizz(x)
    x % 3 == 0

let is_buzz(x)
    x % 5 == 0

let fizzbuzz (x)
    switch 
    case is_fizzbuzz(x)
       fizzbuzz_strings[2]
    case is_fizz(x)
       fizzbuzz_strings[0]
    case is_buzz(x)
       fizzbuzz_strings[1]
    default
       x

for i in 1 til 100
    console.log fizzbuzz(i)
