# `webster`

An application that tries to solve a Wordle puzzle only by using
clues. You decide the word to discover (if you want to compete with
`webster`, use the word you already solved for the day.) As `webster`
provides guesses, you need to give the clues.

NOTE: If you give an incorrect clue, you'll have to start over because
`webster`'s game state will have been permanently "damaged".

## To-do items

- [ ] Make `webster` use all three forms of clues
  - [ ] Handle GREEN hints
  - [ ] Handle YELLOW hints
  - [ ] Handle BLACK hints
- [ ] When `webster` correctly guesses, emit the colored-box output
  that Wordle uses to show your friends how you did, without giving
  away any part of the puzzle.
- [ ] If it's too hard to give perfect clues, `webster` can performs
  both roles: guessing and generating the clues.
