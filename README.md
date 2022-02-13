# `webster`

An application that tries to solve a Wordle puzzle only by using
clues. You decide the word to discover (if you want to compete with
`webster`, use the word you already solved for the day.) As `webster`
provides guesses, you need to give the clues.

NOTE: If you give an incorrect clue, you'll have to start over because
`webster`'s game state will have been permanently "damaged".

## To-do items

- [ ] Make `webster` use all three forms of clues
  - [X] Handle GREEN hints
  - [ ] Handle YELLOW hints
  - [ ] Handle BLACK hints
- [ ] When `webster` correctly guesses, emit the colored-box output
  that Wordle uses to show your friends how you did, without giving
  away any part of the puzzle.
- [ ] If it's too hard to give perfect clues, `webster` can performs
  both roles: guessing and generating the clues.

## Progress

As of this commit, `webster` only uses green hints. It can take awhile
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

As more hints are used, we'll re-run it using this word.
