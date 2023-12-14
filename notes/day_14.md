# Day 14

Implementing a cache in part 2 reduces the number of times the tilt function is called (399972 cache hits out of 400000 for 100000 spin cycles). However, this is insufficient to reduce the timing significantly.

Instead a cache is probably needed for the cycles. Once there is a cache hit, it means that there is a cycle. We can then find the length of the cycle and skip the remaning large cycles.

Implmenting the outer cache without any cycle detection resulted in:
 - 0 inner cache hits out of 28 attempts
 - 999993 outer cache hits out of 1000000 attempts

In other words the inner cache is practically useless and the outer cache doesn't reduce the runtime sufficiently.
