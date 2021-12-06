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

### Day 5 

Today was interesting, as I had to make the code worse (ie, skip diagonals) in the first part - but still interesting. Understanding some of the rust differences with inner functions and inner closures (the latter having access to variables) was useful - Probably some of the past code can be simplified with declared closures like these.

I also discovered lazy_static, wherin a function can own a value - in this case, an initialized regex it needs - which obviates the need for some other data structure. 

In other languages, you'd declare that at a module level (perhaps) or otherwise be able to reference it, but in Rust that is a no-go - lazy_static uses macros to make it seem more natural, though behind the scenes there's still an extra holding struct as you may expect. 

### Day 6

I hit [this issue](https://github.com/rust-lang/rust/issues/59159) today, wherin a line can attempt to borrow as both mutable and immutable at the same time:

```
warning: cannot borrow `fish` as mutable because it is also borrowed as immutable
  --> src/day6.rs:38:21
   |
36 | match fish.get(&age) {
   |       ---- immutable borrow occurs here
37 |     Some(fish_count) => {
38 |         fish.insert(age_minus_one, *fish_count);
   |         ^^^^                       ----------- immutable borrow later used here
   |         |
   |         mutable borrow occurs
```

One solution here is to declare a variable which dereferences the fish_count value, then use that in the call to insert:

```
   match fish.get(&age) {
       Some(fish_count) => {
-          fish.insert(age_minus_one, *fish_count);
+          let minus_one_value = *fish_count;
+          fish.insert(age_minus_one, minus_one_value);
           fish.remove(&age);
       }
       None => {
            if fish.get(&age_minus_one).is_some() {
                fish.remove(&age_minus_one);
            }
       }
   }
```

If I understand correctly, this works because the value is copied out of the old reference before being placed into the new one.

Can you swap the references somehow? [This post](https://stackoverflow.com/questions/65580524/how-to-update-a-key-in-a-hashmap) suggests no but there is a neater solution than using match as I had previously:

```
if let Some(fish_count) = fish.remove(&age) {
    fish.insert(age_minus_one, fish_count);
} else {
    // remove if it's there.
    fish.remove(&age_minus_one);
}
```

