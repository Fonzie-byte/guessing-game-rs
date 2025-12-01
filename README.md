> Migrated to [https://codeberg.org/Fonzie-byte/guessing-game-rs](https://codeberg.org/Fonzie-byte/guessing-game-rs)

# guessing-game-rs
My version of the guessing game from the Rust Book chapter 2.

## Sample runs

### Sample winning runs
```
$ guessing_game 
The computer is thinking of a number between 1 and 100, you have 6 chances to guess it.
Please type your guess:
50
50 is too low! 5 chances left!
Please type your guess:
75
75 is too high! 4 chances left!
Please type your guess:
63
63 is too high! 3 chances left!
Please type your guess:
56
56 is too low! 2 chances left!
Please type your guess:
59
59 is too low! 1 chances left!
Please type your guess:
61
You got it! It's 61!

$ guessing_game 
The computer is thinking of a number between 1 and 100, you have 6 chances to guess it.
Please type your guess:
50
You got it! It's 50!
```

### Sample losing run
```
$ guessing_game 
The computer is thinking of a number between 1 and 100, you have 6 chances to guess it.
Please type your guess:
50
50 is too high! 5 chances left!
Please type your guess:
0
0 is too low! 4 chances left!
Please type your guess:
100
100 is too high! 3 chances left!
Please type your guess:
90
90 is too high! 2 chances left!
Please type your guess:
80
80 is too high! 1 chances left!
Please type your guess:
70
70 is too high! 0 chances left!
Too bad... The number was 28.
```

## Strategy tips
- Too high = Take the average between the last guess and the highest guess so far or 100; fill in the nearest integer of that number.
- Too low = Take the average between the last guess and the lowest guess so far or 0; fill in the nearest integer of that number.

### Following along with the first example run:

| # | Result          |      Calc optimal next move |
|--:|-----------------|----------------------------:|
| 1 | 50 is too low!  |           `(50+100)÷2 = 75` |
| 2 | 75 is too high! |          `(50+75)÷2 = 62.5` |
| 3 | 63 is too high! |       `(50+62.5)÷2 = 56.25` |
| 4 | 56 is too low!  |   `(56.25+62.5)÷2 = 59.375` |
| 5 | 59 is too low!  | `(59.375+62.5)÷2 = 60.9375` |
