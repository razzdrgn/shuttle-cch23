# 🚀 Shuttle.rs Christmas Code Hunt 2023 Submissions 🎄

These are my submissions for the [Christmas Code Hunt](https://www.shuttle.rs/cch) hosted by [shuttle.rs](https://shuttle.rs/).

This is using a [custom service](https://docs.shuttle.rs/tutorials/custom-service) (even though it's a tad unnecessary) to wrap around an [Axum runtime](https://github.com/tokio-rs/axum).

Live host is available at https://console.shuttle.rs/project/cch23-razzdrgn
**(NOTE: HAS NOT BEEN UPDATED SINCE DEC 01 DUE TO SERVICE DISRUPTIONS)**

This README contains my notes on each day's challenge, while the [wiki](https://github.com/razzdrgn/shuttle-cch23/wiki) has documentation about each endpoint, its inputs and outputs, and what should be returned.

Any feedback on the codebase is appreciated, since I'm very much a webdev beginner, and intermediate with Rust.

## Day 0

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-0)

This day was mainly for getting the project set up, and dipping my toes into axum, since it's my first time using it. I still wrapped the dang thing in a custom service anyway because you never know when you'll need to implement an extra runtime for whatever functionality.

## Day 1

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-1)

This one was interesting, because while I got it to work pretty quickly using a for loop, I talked to some folks in the community also working on the challenge and learned about how to use the `.map` functions with closures. Needless to say it helped a lot with simplifying it and making it easier to read, and (writing this in the future from then) definitely helped me for the challenges ahead.

## Day 4

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-4)

I feel like I remember this one being harder than it was, if only because of all the nonsense I was trying to do with my closures. Obviously after learning them the previous friday I immediately had to use them, and I did some Wacky Shit to get the results to work properly.

The biggest things I learend here were [serde_json's](https://github.com/serde-rs/json) `Value` type, and it's `#[serde(rename(/* ... */))]` macro. The actual logic is... A bit rough.

The output of the contest endpont, for the record, is the Strength and Name of the deer with the highest Speed, the Antler Width and Name of the deer with the highest Height, the Name and 'Snow Magic Power' of the deer with the highest 'Snow Magic Power', and the Name and Favorite Food of the deer with the highest 'Candies Eaten Yesterday' value. It's formatted in a human-readable way inside the JSON, though if this were to (hypothetically) be a production endpoint, I'd just return the name key of each deer since that's what's likely being used as an index.

## Day 6

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-6)

So while there's really nothing special to say since this is just running some super basic regex on the input text, the problems for this day (which stretched on for far longer than they needed to) were caused when validating my code with the various validators that existed over the last few weeks. The specific expectations for the outputs of each test case varied between the implementations, and a lot of the edge cases were a little absurd, leading to me having to rewrite the logic on this at least 10 times. Probably more.

The specific integer values being returned are the number of matches of the word "elf", the phrase "elf on a shelf", and lone instances of the word "shelf" with no "elf on a" preceding. You can check the source to see the specific implementation, but based on the validators this was tested against, it should work fine.

## Day 7

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-7)

This one was really fun to work on actually! Mostly because for the final validation, I had to kind of invert my method of thinking to solve it.

To calculate the number of cookies possible, I take the minimum value of the number of recipe items in the pantry over the number required by the recipe. Originally I was iterating over the pantry, thinking that it would make more sense since I'm subtracting/dividing the ingredients FROM the values in the pantry, but in order to parse situations where there's items in the recipe that aren't in the pantry, and vice versa, or when there's zero of items, I had to iterate over the recipe instead.

Iterating over the pantry made more sense for then subtracting the necessary items from it, but those extra edge cases only needing a simple change in the code made me happy, and I'm pretty proud of myself for managing to get everything to work so tightly.

## Day 8

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-8)

I spent a not insignificant time trying to get this to work using things that were already included within Axum, since in theory it should be able to send out get requests. However, I couldn't figure it out, so I ended up just using [reqwest](https://github.com/seanmonstar/reqwest), which is way more boring, but also way easier.

## Day 10

No challenge today, but went through and tried fixing things to work with the unofficial [codehunt.rs](https://www.codehunt.rs) leaderboard. Big things are just, better usage of serde across the board, and a few minor logic fixes, but no major changes, just fixing json fields. My stuff still isn't working with their API though because:

1. ngrok keeps giving me network errors
2. when the data does arrive it arrives corrupted

I've manually checked all of the results by hand and like. My code's working. There's just something Going On that I'm not sure what it is. I'm hoping to work with the guy who's running it at some point to figure out what it is so here's hoping.

## Day 11

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-11)

I'll be real, the only interesting thing was implementing the `/11/red_pixels` logic. I spent, again, a very embarassing amount of time trying to iterate over the raw bytes of the image, before giving up and using the [image](https://github.com/image-rs/image) crate. Even once I got it there, actually processing over the images, and converting the bytes into something that wouldn't overflow, was a pain in the ass. However, today I figured you could pass functions directly into `.map`, which was nice, and helped a bit with making the code a bit easier to read.

Also implemented various fixes to work with the new offline validator:

- /-1 is now /-1/error
- Day 6 regex is fixed, and reduced processing intensity by only using 1 regex call
- Day 7 logic now iterates over the recipe to filter out 0-valued recipe ingredients

As a fun bonus, also managed to fix the network errors with ngrok. That was more an issue with my environment than anything else, but hey, stuff working is always good.

## Day 12

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-12)

I had to learn how to use Axum's shared state today, and it was. Okay? The issue was dealing with all the `Arc`s and the `RwLock`s and the containers you have to stuff the actual state into. Once I figured it out it was pretty smooth sailing from there, though trying to convert between [chrono](https://github.com/chronotope/chrono) and `std::time` was interesting. Not fun, but interesting.

## Day 13

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-13)

I started on this one late since I was out and about today, and having to suddenly learn how to use Docker, and [sqlx](https://github.com/launchbadge/sqlx), and the nightmare of trying to have a shared state across nested routes stored within modules, today was painful. But GOD DAMN am I happy with how this turned out. Like, holy shit. This is some of the cleanest code I've written yet, it worked basically first try, and man. I am just proud of myself. I don't know if it's the best or cleanest or what implementation of this problem, but I'm extremely happy with how this turned out.

I was so happy that, as part of getting the postgres database to work, I refactored basically every module's router function to take better advantage of axum's nesting features. In hindsight I should have done that AFTER everything was fully working and I was proud of myself, but yknow? Sometimes you have to do something NOW, and since I didn't know how to best make the router functions nest properly while also sharing state, I just. Did it right then. Live and learn.

## Day 14

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-14)

There's really not much to say about this challenge, it was pretty straightforward. The problems come, again, with the validators. The html was expected to be space-indented, and I am a member of the tab-indent party. Also, in order to get the data to actually send to codehunt.rs, I had to sign up for an ngrok account. Which, boooo, they didn't let me use SimpleLogin for email obscuring, then immediately I got bullshit marketing emails from them. Yuck. At least the problem worked well.

## Day 15

[Endpoint Documentation](https://github.com/razzdrgn/shuttle-cch23/wiki/Day-15)

Today's challenge implementation was so huge that I had to break out a whole ass module folder for it. Wow!

So, this is basically just doing regex matching on inputs, then processing each step in an if/else ladder. Really, nothing super complicated. But god damn did I hate how long the code file was. It was just a huge function with a shit ton of if/elses, and I didn't like that implementation.

So, after getting the `/15/nice` endpoint to work, I went and made a whole new module. The `passwordgame.rs` module is basically all the actual business logic for this implementation. I made a serde compatible enum for all the different possible results and response messages, I made each individual test into its own function, then I made the if/else ladder that calls those functions into its own private function, and if THAT wasn't enough I put the actual Axum Response into a function inside the struct!

Like, realistically, this is too much, but this is also an exercise in building strong, resiliant code the first time. Because the only things that didn't actually work immediately, were the regex strings used, and the fact that I didn't invert the logic when designing it in my head. Literally, other than those two things, this worked flawlessly, and today is another day I'm proud of myself for.
