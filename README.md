# Connect 4
A Rust program to play (decent) Connect 4.

## How it works
The program looks a few moves (aka turns) into the future (five on my computer) and when it reaches each of those future positions, it needs to determine which of those positions are best.
It does this by playing a few hundred games, starting at each of the positions and playing random moves, until the game is over, be it won or drawn. For each win, the "position" gains a point.
For each draw, it gains nothing and for each loss it loses a point. The average of all the points in the position is calculated. 
This average score represents the expected value of the position, which can be calculated by `win% - lose%`. The yellow player aims to maximize this number, whilst the red player aims to minimize this number.
That means a score of `0.3` means "the yellow player wins 30% more games than they lose", whilst a score of `-0.9` means "the red player wins 90% more games than they lose". 
These scores may be displayed either as a decimal (between -1 and 1) or as a percentage (between -100% and 100%).

## Representing the Board
I chose to use bitboards to represent the board. This has its benefits and drawbacks:
### Benefits
- It is stupendously easy and fast to manipulate bits and therefore perform various board transformations, such as determining a line of four yellow or red pieces to find a winner.
- I already have plenty of experience in bitboards thanks to my Chess Engine programs, one of which being [Tjangas](https://github.com/nicholasdejong/tjangas).
- Representing a Connect 4 board is as easy and cheap as two 64-bit unsigned integers (U64's): one for each colour.
  - A bitboard is represented as a U64.
  - A bitboard can store 64 squares of information, with each square capable of having two states: occupied or empty in our case.
  - Since there are two colours, a bitboard representing the red pieces being empty at a square means "There may be yellows here but there are no reds."
  - To find all occupied squares, all you have to do is `red_bitboard | yellow_bitboard`, where `|` represents the bitwise `OR` operator.
 
### Drawbacks
- The most immediately evident drawback (if you are familiar with Connect 4) is that my program's board representation is 8 columns by 8 rows wide (8x8).
  - The standard game is played on a 7x6 board, and since it has been proven to be a win for the starting player, it is (probably*) also a win for the first player on an 8x8 board.
  - Implementing Connect 4 on a 7x6 board is less elegant and also requires a bit more work, with most of the bitwise logic requiring replacement.
  - A 42-bit unsigned integer (U42) will be necessary. 

## The technical details

### Development process
As I implement different features for the program, I tend to separate these features on different branches. There are currently four branches:
- `master`: The branch where I intend to merge my features into a centralized area for guests to browse, as well as containing the main documentation for this project.
- `interface`: The branch focusing on developing an interface for interacting with the Connect4 engine. Currently, I use it to benchmark my different iterations of the engines as I try to improve them. I am encountering some drawbacks during this process that I discuss below. In the future, this branch will contain nothing but the interface for the project, meaning if you want to interact with the engine in a more convenient way or pair it against itself, you'd clone this branch, and if you'd want the engine exclusively then you'd clone the branch representing the engine, which right now is `mcts` but is subject to change.
- `mcts`: I plan to merge this branch with the master branch soon, since this branch contains most of my work on the engine. The implementation makes use of Rust's `Rc` and `RefCell` types to ensure interior mutability of the Monte-Carlo game tree.
- `zipper`: I experimented with an alternative approach towards representing the game tree, discovering this very clever data structure. However, for some reason the performance takes a huge hit as it stands, so I plan on profiling the code in the future. But this is the problem I am facing right now.

### Profiling
For me, trying to optimize performant software in general is sort of a mystery to me at the moment. I mostly rely on [flamegraph](https://github.com/flamegraph-rs/flamegraph) to give me a general gist of what my program is spending time on. This is still very useful, especially for recursive programs, since certain blocks of code are much "hotter" or "valuable" time-wise because they run multiple times. But as useful as flamegraph is, I wanted better. I wanted to know what parts of code have the best returns (and which are actually worth optimizing) and the effect the optimizations will have on the program. This is when I found a great talk by Emery Berger on ["Performance Matters"](https://www.youtube.com/watch?v=r-TLSBdHe1A&pp=ygUTcGVyZm9ybWFuY2UgbWF0dGVycw%3D%3D). They introduced a causal profiler known as `coz` which essentially answers the exact questions I want to know. Unfortunately I have been unsuccessful getting it to run on my Arch Linux (😅) machine. Currently I am experiencing [this issue](https://github.com/plasma-umass/coz/issues/233).

### Contributing
As a single developer, a second pair of eyes are incredibly useful. I strongly encourage you to share your thoughts or suggestions by creating [an issue](https://github.com/nicholasdejong/connect4/issues/new/choose). It can be as small as a single typo or as large as half the project! Pull requests are also welcome and encouraged. Thank you for taking an interest in this repository ❤️
