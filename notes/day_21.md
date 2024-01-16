In exactly n steps, the gardener can only stop at positions with an even manhattan distance away from the start if n is even. The logic is similar if n is odd.

Basically we can just do a normal search and keep only values at a even manhattan distance.

Instead of calculating the distance, I found it easier to just use the parity on the step.

Part 2 is going to require reimplementing the search to work with isize to accomodate values out of the grid and wrap around to identify the untraversible spaces.