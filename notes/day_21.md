In exactly n steps, the gardener can only stop at positions with an even manhattan distance away from the start if n is even. The logic is similar if n is odd.

Basically we can just do a normal search and keep only values at a even manhattan distance.

Instead of calculating the distance, I found it easier to just use the parity on the step.

Part 2 is going to require reimplementing the search to work with isize to accomodate values out of the grid and wrap around to identify the untraversible spaces.

Actually implementing the infinite grid wasn't actually so bad.

However, the issue is that the search is taking too long given the size of the search. I don't think I'm doing anything too extra here. But I think in order to optimize this I need to make use of the repeating grid somehow.

What I'm thinking is that the inside doesn't really need to be searched. I just need to look for the parameter of the reachable plots. There is also the issue of identifying the parity too.