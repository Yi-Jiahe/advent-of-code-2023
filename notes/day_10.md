# Day 10

I made the position a tuple of signed integers because I figured that it would be useful to be to accept values outside the grid. 
That turned out to be more trouble than it was worth because when I needed to access something in the grid I had to cast it to an insigned integer.
While the constraints of the logic ensure that the cast is valid, its a little iffy.

I would refactor them to use only unsigned ints but I've spent long enough as it is.