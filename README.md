# adventofcode2021

This year in [Rust](https://www.rust-lang.org/)

As it's been quite a few years since I've written any Rust, I set up a [devcontainer](https://code.visualstudio.com/docs/remote/create-dev-container) for vscode (defaults) to compile with.

After initially running into some issues with the snap-installed version of docker and apparmor, switching to the non-snap docker.io package has things running fairly smoothly.

## Note!

I'm probably implementing these things *all wrong* so if you happen to be reading this feel free to tell me how wrong and what do to instead!.

## Note on puzzle input data

Since AOC's input data is different for everyone, you won't be able to just run this code and have it create the right responses for your own submissions.

## Commentary

### Day 4

Quite a challenging day. While conceptually creating a struct to represent each bingo card seemed like the cleanest implementation, understanding the rules around borrowed and mutable objects took quite some time to get to a point where the implementation worked as expected (without the compiler complaining).

I followed some advice I read online about clone()-ing liberally when first learning Rust, and this has gotten me to a result. Looking at some of that code I would hope you'd be able to optimize and not clone() if/when I understand how this works a bit better.