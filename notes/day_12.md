# Day 12

Surprisingly the iterations are taking quite a long time. Generating a number in the order of a few hundred thousand takes about a minute and a number in the millions takes easily 20 minutes.

Perhaps treating the strings as 1 byte arrays instead of arbitrary length characters in order to avoid having to iterate the string would help. The characters are all ASCII characters after all.

Reducing the number of states to explore would also be good.

Using a cache might help too but it would likely require me to figure out how to describe each state and rewrite the function definitions.