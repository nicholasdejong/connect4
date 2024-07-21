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
TODO


\* Please correct me on this.
