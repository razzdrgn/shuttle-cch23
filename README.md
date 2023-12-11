# 🚀 Shuttle.rs Christmas Code Hunt 2023 Submissions 🎄

These are my submissions for the [Christmas Code Hunt](https://www.shuttle.rs/cch) hosted by [shuttle.rs](https://shuttle.rs/)

This is using a [custom service](https://docs.shuttle.rs/tutorials/custom-service) (even though it's a tad unnecessary) to wrap around an [Axum runtime](https://github.com/tokio-rs/axum).

Live host is available at https://console.shuttle.rs/project/cch23-razzdrgn
**(NOTE: HAS NOT BEEN UPDATED SINCE DEC 01 DUE TO SERVICE DISRUPTIONS)**

## Day 0 (Implemented Nov 30, 2023)

Returns 200 and a fun xmas message at the root endpoint.
Returns 500 and a silly bonus message at the /-1/error endpoint

## Day 1 (Implemented Dec 01, 2023)

Decodes path from the /1 endpoint onwards and performs a Bitwise XOR operation on the numbers provided, or on 0 if the parsing fails.
After the Bitwise XOR, raises the result to the power of 3 and then returns the result along with status 200.

## Day 4 (Implemented Dec 04, 2023)

Takes POST requests containing a JSON array of Reindeer objects. At the endpoint /4/strength returns the sum total of the "strength" attributes.
At the endpoint /4/contest, performs some comparisons on the properties of the objects, then returns user-readable output containing stats about the inputs.
Namely, the reindeer with the highest "speed", "height", "snow_magic_power", and "candies_eaten_yesterday" attributes.

## Day 6 (Implemented Dec 06, 2023)

Takes a POST request containing plaintext, and returns a JSON object containing the counts of the occurances of
"elf", "elf on a shelf", and "shelf" (without "elf on a " preceding it).

## Day 7 (Implemented Dec 07, 2023)

Endpoint /7/decode analyzes the Cookie header, and decodes the "recipe" field from base64 into a JSON object.
Endpoint /7/bake takes in said recipe JSON objects (with properties "recipe" and "pantry"), then tries to "bake" the cookies.
For the process to work, the Recipe's components must all exist in the Pantry, and the Pantry must contain equal to or greater than the number of materials required by the recipe.

## Day 8 (Implemented Dec 08, 2023)

Endpoint /8/weight will take a pokedex number from the path (i.e. `GET .../8/weight/150` for Mewtwo) and return the Pokemon's weight in Kg as plaintext.
Endpoint /8/drop will take the same parameters and return the impact momentum from a 10 foot freefall, returned as N*s in plaintext.

## Day 10

No challenge today, but went through and tried fixing things to work with the unofficial [codehunt.rs](https://www.codehunt.rs) leaderboard. Big things are just, better usage of serde across the board, and a few minor logic fixes, but no major changes, just fixing json fields. My stuff still isn't working with their API though because:

1. ngrok keeps giving me network errors
2. when the data does arrive it arrives corrupted

I've manually checked all of the results by hand and like. My code's working. There's just something Going On that I'm not sure what it is. I'm hoping to work with the guy who's running it at some point to figure out what it is so here's hoping.

## Day 11 (Implemented Dac 11, 2023)

Added an assets server under the /11/assets endpoint that returns assets within the folder "assets", using Tower's `services::ServeDir` service.
Endpoint /11/red_pixels counts the number of pixels in an image where the red value is greater than the sum of the green and blue values.

Also implemented various fixes to work with the new offline validator:
- /-1 is now /-1/error
- Day 6 regex is fixed, and reduced processing intensity by only using 1 regex call
- Day 7 logic now iterates over the recipe to filter out 0-valued recipe ingredients

As a fun bonus, also managed to fix the network errors with ngrok. That was more an issue with my environment than anything else, but hey, stuff working is always good.