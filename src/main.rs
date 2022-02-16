use std::collections::HashMap;
use std::io::{self, Write};

mod dictionary;

#[derive(PartialEq, Clone, Debug)]
enum Hint {
    Black,
    Yellow,
    Green,
}

impl TryFrom<char> for Hint {
    type Error = ();

    fn try_from(ch: char) -> Result<Hint, ()> {
        match ch {
            'G' | 'g' => Ok(Hint::Green),
            'Y' | 'y' => Ok(Hint::Yellow),
            'B' | 'b' => Ok(Hint::Black),
            _ => Err(()),
        }
    }
}

// Holds character frequency information. This type is meant to be fed
// a stream of Hints; the first is fed to `new()` and the rest to
// `update()`. The value will keep track of how many of the character
// could be in the word, based on the hints.

#[derive(Debug, PartialEq)]
struct FreqInfo(usize, usize);

impl FreqInfo {
    // Create a new `FreqInfo` with an initial hint.

    pub fn new(hint: &Hint) -> FreqInfo {
        if *hint == Hint::Black {
            FreqInfo(0, 0)
        } else {
            FreqInfo(1, 5)
        }
    }

    // Updates the possible range of totals of the character based on
    // the hint passed to it. Note the upper limit will be
    // conservatively high because each `FreqInfo` has no access to
    // other frequency measurements.

    pub fn update(&mut self, hint: &Hint) {
        match hint {
            Hint::Black => self.1 = self.0,
            Hint::Yellow | Hint::Green => {
                self.0 += 1;
                self.1 = std::cmp::max(self.0, self.1)
            }
        }
    }
}

// Returns hints given by the user. The loop is so the input can be
// re-entered if the user entered something invalid.

fn get_hints() -> io::Result<String> {
    loop {
        let mut input = String::new();

        // Prompt the user and get the hints.

        print!("   Hints> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        // Remove trailing whitespace and make everything uppercase so
        // we don't have to test for lowercase hints.

        let input = input.trim().to_uppercase();

        // The input *must* be 5 characters.

        if input.len() != 5 {
            println!("ERROR: hints must contain 5 characters");
            continue;
        }

        // The input can only contain the letters B, Y, and G.

        if input.matches(|c| c == 'B' || c == 'Y' || c == 'G').count() < 5 {
            println!("ERROR: only letters in hints are B, Y, and G");
            continue;
        }

        return Ok(input);
    }
}

// Uses the green and yellow hints to reduce the vocabulary. For a
// green hint, the GreenTable is used to find all words with the
// character in the position. The vocabulary is ANDed (i.e. the
// intersection) with the set of words which will remove words that
// don't have that condition. For yellow hints, we remove all words
// with the character in the position from the vocabulary. This
// preserves words with the character -- just not in the position.

fn process_position_hints(
    vocab: &mut dictionary::Words, gt: &dictionary::GreenTable, guess: &str,
    hints: &Vec<Hint>,
) {
    // Turn the guess and hints into a (idx, hint, guess char)
    // iterator.

    let iter = hints
        .iter()
        .enumerate()
        .zip(guess.chars())
        .map(|((idx, hint), ch)| (idx, hint, ch));

    // Loop through the hint/guess items and process each.

    for (idx, hint, ch) in iter {
        // This algorithm doesn't handle Black hints.

        if *hint != Hint::Black {
            let words = gt.get(&(idx, ch)).unwrap();

            // If it was a Green hint, compute the intersection of the
            // vocabulary with the set of words having the character
            // in the current position.

            if *hint == Hint::Green {
                vocab.preserve(words)
            } else {
                // It's a Yellow hint. Build up a set of words that
                // have the current character in every position *but*
                // the current one.

                let mut keep_words = dictionary::Words::new(&[]);

                for ii in 0..=4 {
                    if ii != idx {
                        if let Some(tmp) = gt.get(&(ii, ch)) {
                            keep_words.union(tmp);
                        }
                    }
                }

                // Compute the intersection of the vocabulary with the
                // words containing the current character *not* in the
                // current position.

                if keep_words.total() > 0 {
                    vocab.preserve(&keep_words);
                }

                // Remove all words from the vocabulary where the
                // current character is in the current position.

                vocab.remove(words)
            }
        }
    }
}

fn bld_freq_info_table(
    hints: &Vec<Hint>, guess: &str,
) -> HashMap<char, FreqInfo> {
    let mut freq = HashMap::<char, FreqInfo>::new();

    // Build the table of char -> freq info.

    for (hint, ch) in hints.iter().zip(guess.chars()) {
        if let Some(info) = freq.get_mut(&ch) {
            info.update(hint);
        } else {
            freq.insert(ch, FreqInfo::new(hint));
        }
    }

    // Make one more pass through the table and adjust the upper bound
    // of each entry.

    let maxes: Vec<(char, usize)> = freq
        .iter()
        .filter(|(_, v)| v.0 > 0)
        .map(|(k, v)| (*k, v.0))
        .collect();

    for (ii_k, ii_v) in freq.iter_mut() {
        let mut total = 0;

        for (jj_k, jj_v) in &maxes {
            if jj_k != ii_k {
                total += jj_v
            }
        }

        ii_v.1 = std::cmp::min(ii_v.1, 5 - total)
    }

    freq
}

// Use the clues to reduce the vocabulary.

fn process_hints(
    mut vocab: dictionary::Words, gt: &dictionary::GreenTable,
    ft: &dictionary::CharFreqTable, guess: &str, hints: &Vec<Hint>,
) -> dictionary::Words {
    process_position_hints(&mut vocab, gt, guess, hints);

    let freq = bld_freq_info_table(hints, guess);
    let mut keep_words = dictionary::Words::new(&[]);

    for (ch, FreqInfo(l, h)) in freq.iter() {
        for ii in 1..=5 {
            if let Some(tmp) = ft.get(&(ii, *ch)) {
                if ii < *l || ii > *h {
                    vocab.remove(tmp);
                } else {
                    keep_words.union(tmp);
                }
            }
        }
    }

    if keep_words.total() > 0 {
        vocab.preserve(&keep_words);
    }

    vocab
}

// Preps the hint tables and the initial vocabulary. Then it enters
// the main loop of the program where it picks a random word from its
// vocabulary, waits for clues, then applies them to its vocabulary.

fn main() -> io::Result<()> {
    // Prep the hint tables and start with the full vocabulary.

    let mut vocab = dictionary::get_vocabulary();
    let grn_tbl = dictionary::GreenTable::new();
    let frq_tbl = dictionary::CharFreqTable::new();

    let mut guess = "irate";

    loop {
        if vocab.total() < 20 {
            println!(
                "(vocab: {:?})\nMy guess: {}",
                vocab,
                guess.to_uppercase()
            );
        } else {
            println!(
                "(vocabulary: {} words)\nMy guess: {}",
                vocab.total(),
                guess.to_uppercase()
            );
        }

        // Get hints from the user.

        let input = get_hints()?;

        // If it's GGGGG, then the guess was correct.

        if input == "GGGGG" {
            println!("Solved it! The word was \"{}\"", guess.to_uppercase());
            break;
        }

        // Convert the hint string into an array of Hint types.

        let hints = input.chars().map(|c| Hint::try_from(c).unwrap()).collect();

        // Reduce the vocabulary by applying the hints.

        vocab = process_hints(vocab, &grn_tbl, &frq_tbl, guess, &hints);

        if vocab.total() == 0 {
            println!("I'm out of words. Did you make a mistake with a clue?");
            break;
        }

        // Pick a random word from the vocabulary. This will be the
        // guess for this iteration of the loop.

        guess = vocab.pick_random()
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn apply_state(hints: &Vec<Hint>) -> FreqInfo {
        let mut info = FreqInfo::new(&hints[0]);

        for hint in &hints[1..] {
            info.update(hint)
        }
        info
    }

    #[test]
    fn test_freq_info() {
        assert_eq!(apply_state(&vec![Hint::Black]), FreqInfo(0, 0));
        assert_eq!(apply_state(&vec![Hint::Yellow]), FreqInfo(1, 5));
        assert_eq!(apply_state(&vec![Hint::Green]), FreqInfo(1, 5));

        assert_eq!(
            apply_state(&vec![Hint::Black, Hint::Black]),
            FreqInfo(0, 0)
        );
        assert_eq!(
            apply_state(&vec![Hint::Black, Hint::Yellow]),
            FreqInfo(1, 1)
        );
        assert_eq!(
            apply_state(&vec![Hint::Black, Hint::Green]),
            FreqInfo(1, 1)
        );

        assert_eq!(
            apply_state(&vec![Hint::Yellow, Hint::Black]),
            FreqInfo(1, 1)
        );
        assert_eq!(
            apply_state(&vec![Hint::Yellow, Hint::Yellow]),
            FreqInfo(2, 5)
        );
        assert_eq!(
            apply_state(&vec![Hint::Yellow, Hint::Green]),
            FreqInfo(2, 5)
        );

        assert_eq!(
            apply_state(&vec![Hint::Green, Hint::Black]),
            FreqInfo(1, 1)
        );
        assert_eq!(
            apply_state(&vec![Hint::Green, Hint::Yellow]),
            FreqInfo(2, 5)
        );
        assert_eq!(
            apply_state(&vec![Hint::Green, Hint::Green]),
            FreqInfo(2, 5)
        );

        assert_eq!(
            apply_state(&vec![Hint::Black, Hint::Black, Hint::Black]),
            FreqInfo(0, 0)
        );
        assert_eq!(
            apply_state(&vec![Hint::Black, Hint::Yellow, Hint::Black]),
            FreqInfo(1, 1)
        );
        assert_eq!(
            apply_state(&vec![Hint::Black, Hint::Green, Hint::Black]),
            FreqInfo(1, 1)
        );

        assert_eq!(
            apply_state(&vec![Hint::Green, Hint::Yellow, Hint::Black]),
            FreqInfo(2, 2)
        );
    }

    #[test]
    fn test_freq_info_table() {
        {
            let tbl = bld_freq_info_table(
                &vec![
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                ],
                "abcde",
            );
            let mut expected: HashMap<char, FreqInfo> = HashMap::new();

            expected.insert('a', FreqInfo(0, 0));
            expected.insert('b', FreqInfo(0, 0));
            expected.insert('c', FreqInfo(0, 0));
            expected.insert('d', FreqInfo(0, 0));
            expected.insert('e', FreqInfo(0, 0));

            assert_eq!(tbl, expected);
        }
        {
            let tbl = bld_freq_info_table(
                &vec![
                    Hint::Yellow,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                ],
                "abcde",
            );
            let mut expected: HashMap<char, FreqInfo> = HashMap::new();

            expected.insert('a', FreqInfo(1, 5));
            expected.insert('b', FreqInfo(0, 0));
            expected.insert('c', FreqInfo(0, 0));
            expected.insert('d', FreqInfo(0, 0));
            expected.insert('e', FreqInfo(0, 0));

            assert_eq!(tbl, expected);
        }
        {
            let tbl = bld_freq_info_table(
                &vec![
                    Hint::Yellow,
                    Hint::Yellow,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                ],
                "aabcd",
            );
            let mut expected: HashMap<char, FreqInfo> = HashMap::new();

            expected.insert('a', FreqInfo(2, 5));
            expected.insert('b', FreqInfo(0, 0));
            expected.insert('c', FreqInfo(0, 0));
            expected.insert('d', FreqInfo(0, 0));

            assert_eq!(tbl, expected);
        }
        {
            let tbl = bld_freq_info_table(
                &vec![
                    Hint::Yellow,
                    Hint::Yellow,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                ],
                "aaabc",
            );
            let mut expected: HashMap<char, FreqInfo> = HashMap::new();

            expected.insert('a', FreqInfo(2, 2));
            expected.insert('b', FreqInfo(0, 0));
            expected.insert('c', FreqInfo(0, 0));

            assert_eq!(tbl, expected);
        }
        {
            let tbl = bld_freq_info_table(
                &vec![
                    Hint::Yellow,
                    Hint::Yellow,
                    Hint::Black,
                    Hint::Yellow,
                    Hint::Black,
                ],
                "aaabc",
            );
            let mut expected: HashMap<char, FreqInfo> = HashMap::new();

            expected.insert('a', FreqInfo(2, 2));
            expected.insert('b', FreqInfo(1, 3));
            expected.insert('c', FreqInfo(0, 0));

            assert_eq!(tbl, expected);
        }
        {
            let tbl = bld_freq_info_table(
                &vec![
                    Hint::Yellow,
                    Hint::Yellow,
                    Hint::Black,
                    Hint::Yellow,
                    Hint::Black,
                ],
                "aacbd",
            );
            let mut expected: HashMap<char, FreqInfo> = HashMap::new();

            expected.insert('a', FreqInfo(2, 4));
            expected.insert('b', FreqInfo(1, 3));
            expected.insert('c', FreqInfo(0, 0));
            expected.insert('d', FreqInfo(0, 0));

            assert_eq!(tbl, expected);
        }
    }

    #[test]
    fn test_position_hints() {
        {
            let mut vocab =
                dictionary::Words::new(&["aaaaa", "bbbba", "cccac"]);
            let gt = dictionary::mk_green_tbl(&["aaaaa", "bbbba", "cccac"]);
            let expected = dictionary::Words::new(&["aaaaa", "bbbba"]);

            process_position_hints(
                &mut vocab,
                &gt,
                "aaaaa",
                &vec![
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Green,
                ],
            );
            assert_eq!(vocab, expected);
        }

        {
            let mut vocab =
                dictionary::Words::new(&["aaaaa", "bbbba", "cccac"]);
            let gt = dictionary::mk_green_tbl(&["aaaaa", "bbbba", "cccac"]);
            let expected = dictionary::Words::new(&["cccac"]);

            process_position_hints(
                &mut vocab,
                &gt,
                "aaaaa",
                &vec![
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Yellow,
                ],
            );
            assert_eq!(vocab, expected);
        }

        {
            let mut vocab =
                dictionary::Words::new(&["aaaaa", "aacab", "bbbba", "cccac"]);
            let gt =
                dictionary::mk_green_tbl(&["aaaaa", "aacab", "bbbba", "cccac"]);
            let expected = dictionary::Words::new(&["aacab"]);

            process_position_hints(
                &mut vocab,
                &gt,
                "aaaac",
                &vec![
                    Hint::Black,
                    Hint::Black,
                    Hint::Black,
                    Hint::Green,
                    Hint::Yellow,
                ],
            );
            assert_eq!(vocab, expected);
        }
    }
}
