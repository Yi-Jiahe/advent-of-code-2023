# Day 8

My left right instructions are 263 instructions (characters) long.

Interestingly my first answer is 20777 which is 79 * 263, i.e. 79 full sets of the instructions.

In part 2, I memoized the number of steps it takes to get from any visited node to an end node given the current instruction and I realized that all the paths were a multiple of instruction sets apart, i.e. you will always be on the first instruction when you're on a end node. 

```
 &memo = {
    ("LLA", 0): (("NQZ", 0), 17621),
    ("NQZ", 0): (("NQZ", 0), 17621),
    ("MNA", 0): (("GVZ", 0), 18673),
    ("GVZ", 0): (("GVZ", 0), 18673),
    ("NHA", 0): (("DDZ", 0), 11309),
    ("DDZ", 0): (("DDZ", 0), 11309),
    ("JQA", 0): (("SCZ", 0), 13939),
    ("SCZ", 0): (("SCZ", 0), 13939),
    ("FSA", 0): (("PTZ", 0), 15517),
    ("PTZ", 0): (("PTZ", 0), 15517),
    ("AAA", 0): (("ZZZ", 0), 20777),
    ("ZZZ", 0): (("ZZZ", 0), 20777),
}
```

Further observations also show that the start and end nodes nicely paired up, looped to the same end, and very nicely had the same distance between the start and the end and per loop.

| Distance | Multiples of 263 (Length of instructions) |
|---|---|
|20777|79|
|18673|71|
|17621|67|
|15517|59|
|13939|53|
|11309|43|

Lowest common multiple between the multiples is 50,530,847,183. Times the length is 13,289,612,809,129, which is my answer.

My memoized solution was able to get the answer before me though.

If I optimized assuming that this pattern holds for all puzzle inputs, which I think is fair considering its too much of a coincidence not be the case, I could probably get the solution much faster.