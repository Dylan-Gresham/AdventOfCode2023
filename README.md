# AdventOfCode2023

My solutions to the 2023 Advent of Code

# Process

I'll be using Rust for all of the solutions. I'm using Rust because I want to 
gain more experience using the language. I'll start each day by looking at the 
question and coming up with a plan of attack before writing any code.

# Reflections

## Day 1

First day wasn't too bad. I initially had a solution that worked in which I 
read the input file line by line into a `Vec<String>` and then iterated through 
that vector and trimmed out all characters except for numbers at each index and 
stored them as a `Vec<Vec<String>>`. Once trimmed, I took the first and last 
number in each index of the vector and smushed them into one string (e.g. From 
["1", "2"] --> "12"). After that I iterated through each index of the overall 
vec and parsed each index into a u64 then added it to a variable called `sum`. 
At the end of everything, I'd return the `sum` variable.

For part 2 I just added in a thing where before trimming, I'd call replace 9 
times to replace all occurences of a number spelled out with `WordNumWord` 
which would then get trimmed into just the numbers. After that to get both 
parts at the same time, I wrapped everything into a function and passed in a 
bool to say if it's part 1 or part 2. If part 1, skip the replacing, if it's 
part 2 it'd do the replacing.

After confirming that my solutions worked, I should've made a commit but I 
didn't so I'll need to rememebr to do that starting on Day 2. Anyways, after 
confirming my solution worked I found other Rust solutions and the one I liked 
the most was from GitHub user [Dehan Jaco Lamprecht](https://github.com/dehan-jl)
who basically used a single line of code to do everything which I thought was 
amazing. After understanding what was going on I rewrote my solution to match 
his which caused the solution to go from taking roughly 15-20 seconds to taking 
less than 1 second. Yes my solution was actually that slow, I'm slightly 
embarrassed.

## Day 2

### Day Of

I have attempted today's puzzle for about 2 hours so far. I'm not sure if I'm 
parsing incorrectly or what but I can't seem to get through part 1. Tomorrow 
I'm going to watch some videos of people working through it to get a sense for 
what needs to be done and then I'm going to redo my attempt again. Might also 
take the time to restructure my AoC directory into each day being a separate 
module instead of each day being a separate binary, or for each day separating 
part one and two into separate modules instead of functions. We'll see.

## Completion

TODO

