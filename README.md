# `wordlebot`

An application that tries to solve a
[Wordle](https://www.nytimes.com/games/wordle/index.html) puzzle only
by using clues. You decide the word to discover (if you want to
compete with `wordlebot`, use the word you already solved for the
day.) As `wordlebot` provides guesses, you need to give the clues.

NOTE: If you give an incorrect clue, you'll have to start over because
`wordlebot`'s game state will have been permanently altered.

## Running

After cloning the project, run it with

```
$ cargo run
```

There are command line options to enable more information or to change
the theme.

```
Webster 1.0.2
Guesses a word by using Wordle clues

USAGE:
    wordlebot [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

        --limit <LIMIT>
            This sets the limit which decides whether the number of words
            remaining is reported instead of each word.

            [env: WORDLEBOT_LIMIT=]
            [default: 20]

    -t, --theme <THEME>
            Once the word is guessed, it displays a summary of the guesses just
            like the official app. This option allows you to change the color of
            the blocks.

            [env: WORDLEBOT_THEME=]
            [default: normal]
            [possible values: normal, high-contrast]

    -v, --verbose
            Reports how many words are left in its vocabulary, after applying
            all the clues. When the number of words drops below a limit, all the
            remaining words are printed.

    -V, --version
            Print version information
```

### Giving Clues

Giving clues is mostly straightforward: Place a B in the corresponding
position of a letter that isn't in the word. Place a Y in the spot
where the letter is in the word, but its current position is wrong.
Place a G in the spot where the corresponding letter is correct and in
the right spot.

It gets a little difficult when a guess uses a particular letter more
than once. If you find yourself trying to figure out how to provide
proper clues, follow this procedure:

- Assign the Gs first
- Any remaining instances of the letter are handled left to right.
  Assign Ys before Bs.

For instance, if the word was OTTER and the guess was TATTY, the
second T would get the G clue. Then we go left to right with the
remaining Ts so the first T gets the Y clue. Since OTTER doesn't have
any more Ts, the third T gets the B clue. So the clue you would
provide would be YBGBB.

## To-do items

- [X] Make `wordlebot` use all three forms of clues
  - [X] Handle GREEN hints
  - [X] Handle YELLOW hints
  - [X] Handle BLACK hints
- [X] When `wordlebot` correctly guesses, emit the colored-box output
  that Wordle uses to show your friends how you did, without giving
  away any part of the puzzle.
- [ ] If it's too hard to give perfect clues, `wordlebot` can perform
  both roles: guessing and generating the clues.

## Progress

This section shows the progress the program made as more clues were
handled.

### Only Green Clues

For this run, `wordlebot` only used green hints. It can take a while
to solve the puzzle because it only makes progress when the word it
chooses generates green hints. But even this can cause surprising
results. For this run, the word to solve was CAMEL. Here's how one
session went:

```
My guess: QUIRK (vocabulary: 2315 words)
  Result> bbbbb
My guess: BATCH (vocabulary: 2315 words)
  Result> bgbyb
My guess: BAWDY (vocabulary: 304 words)
  Result> bgbbb
My guess: PASTY (vocabulary: 304 words)
  Result> bgbbb
My guess: RAJAH (vocabulary: 304 words)
  Result> bgbbb
My guess: MAMMA (vocabulary: 304 words)
  Result> bggbb
My guess: GAMUT (vocabulary: 10 words)
  Result> bggbb
My guess: RAMEN (vocabulary: 10 words)
  Result> bgggb
My guess: GAMER (vocabulary: 5 words)
  Result> bgggb
My guess: TAMER (vocabulary: 5 words)
  Result> bgggb
My guess: GAMER (vocabulary: 5 words)
  Result> bgggb
My guess: CAMEO (vocabulary: 5 words)
  Result> ggggb
My guess: CAMEL (vocabulary: 2 words)
  Result> ggggg
Solved it! The word was "CAMEL"
```

QUIRK was a terrible, first guess. We should probably make the first
guess one of the several words that have been deemed "good, initial
guesses".

### Green and Partial Yellow Clue Support

For this next run, support for yellow hints was added. It fared better
but really could use the information that black hints provide.

```
My guess: REGAL (vocabulary: 2315 words)
  Result> bybyg
My guess: CHILL (vocabulary: 95 words)
  Result> gbbbg
My guess: CAROL (vocabulary: 7 words)
  Result> ggbbg
My guess: CAVIL (vocabulary: 3 words)
  Result> ggbbg
My guess: CAVIL (vocabulary: 3 words)
  Result> ggbbg
My guess: CAMEL (vocabulary: 3 words)
  Result> ggggg
Solved it! The word was "CAMEL"
```

### Green, Yellow, and Black Clue Support

Here's the run when the Black hints were added:

```
My guess: IRATE
   Hints> bbyby
My guess: GLEAN
   Hints> byyyb
My guess: FELLA
   Hints> byyby
My guess: HAZEL
   Hints> bgbgg
My guess: EASEL
   Hints> bgbgg
My guess: CAMEL
   Hints> ggggg
Solved it! The word was "CAMEL"
```

Despite taking 6 guesses, it didn't repeat itself and the set of
matching words kept getting smaller and smaller (I removed that info
from the output.) It's biggest problem is that it can "recall"
every word instead of the most common, like a person would do. For
instance, a person wouldn't choose HAZEL because "Z" has a low
probability of being used. Nor would one choose FELLA because the
double L burns an opportunity to get a clue for another letter.

### All Clues and Better Word Selection

A version has been committed that picks "better" words. Here's a
session:

```
My guess: IRATE
   Hints> bbyby
My guess: SHEAF
   Hints> bbyyb
My guess: NAMED
   Hints> bgggb
My guess: CAMEL
   Hints> ggggg
Solved it! The word was "CAMEL"
```
