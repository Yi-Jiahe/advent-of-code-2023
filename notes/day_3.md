# Day 3

Given an engine schematic (A rectangular document containing '.'s representing empty(?) space, symbols and numbers), determine which numbers are adjacent to symbols.

I see two options here
1) Load the entire schematic into an array so that we can index it spatially, or 
2) Extract the important data into their own data structures and work with those.

Given that the schematic is a fixed size and we don't yet have information on part two, I think 1 would be sufficient and quicker to implement. The second might better represent the context of the data but we don't yet have the full scope anyway.


Working with strings when the characters don't have an even number of bytes is hard... 