# 🚀 Shuttle.rs Christmas Code Hunt 2023 Submissions 🎄

These are my submissions for the [Christmas Code Hunt](https://www.shuttle.rs/cch) hosted by [shuttle.rs](https://shuttle.rs/)

This is using a [custom service](https://docs.shuttle.rs/tutorials/custom-service) (even though it's a tad unnecessary) to wrap around an [Axum runtime](https://github.com/tokio-rs/axum).

Live host is available at https://console.shuttle.rs/project/cch23-razzdrgn
**(NOTE: HAS NOT BEEN UPDATED SINCE DEC 01 DUE TO SERVICE DISRUPTIONS)**

## Day 0 (Implemented Nov 30, 2023)

Returns 200 and a fun xmas message at the root endpoint.
Returns 500 and a silly boonus message at the /-1 endpoint

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