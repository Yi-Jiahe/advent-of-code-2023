# Day 5

Day 6 was quite straightforward to implement the brute force solution, but the large numbers made it impractical for part 2, where it required a billion(?) iterations.

My brute force solution took maybe a few hours? (I left it running overnight) which isn't a wait-able.

Optimizing the solution was  doozy though. Originally I considered maybe condensing all the category mappings into a single mapping. I figured that if I could figure out how to combine 2 mappings I could simply run them sequentially to get a final mapping. Unfortunately I couldn't wrap my head around it. Instead I worked on the actual ranges that were being processed, taking note of the break points instead of iterating through every possibility. This was still a bit of a mess but more straightforward and ran in seconds. 

I also did a pointless optimization, thinking that perhaps there were repeated ranges resulting in duplicate iterations but there weren't. 