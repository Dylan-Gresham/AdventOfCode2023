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

Finally figured it out. I watched a 
[Python solution from Jonathan Paulson](https://www.youtube.com/watch?v=IWCc11nh2QQ)
which quite frankly left me even more confused as I'm not familiar enough with 
Python to completely understand what he wrote. What that video did help with was 
giving me a process that I could follow on my own to process the input a little 
easier. I ended up just going step by step and doing a ton of `println!()`'s each 
step of the way until I had all my input parsed how I wanted it. Once I got the 
input parsed in a nice way, I did the validating and summing. That was part 1. For 
part 2 I was able to do it without a headache. I didn't realize that it was for ALL 
games at first and not just the valid games so I was only doing the summation for 
valid games instead of all games. Once I realized that mistake I was able to just 
move one little part and it got the answer first try.

## Day 3

### Day Of

It looks like I'm going to be busy for a while longer tonight so I don't know if I'm 
going to have the time to get to Day 3 today (Dec 3, 2023) or not but we'll see.

## Day 4

I did this on December 6th, 2 days late. Part one I was able to pass second try, 
my first try I didn't do one little thing properly which ended up throwing off 
my result by about double what it should be. Part two took me a couple tries but 
once I figured out I was doing one bit of my calculation incorrectly I was able 
to get it. I'm pretty proud that I was able to get everything done in a manner 
that doesn't take ages to process. After completing this day I looked at what 
people were saying on the subreddit and saw that a lot of people were saying that 
their code was taking upwards of 20 minutes to run for part 2 so I was happy that 
my first solution took less than a second to run both parts :)

