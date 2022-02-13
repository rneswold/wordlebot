use std::io::{self, Write};

mod dictionary;

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

// Use the clues to reduce the vocabulary.

fn process_hints(
    mut vocab: dictionary::Words, gt: &dictionary::GreenTable, guess: &str,
    hints: &str,
) -> dictionary::Words {
    for ((idx, hint), ch) in hints.char_indices().zip(guess.chars()) {
        if hint != 'B' {
            if let Some(words) = gt.get(&(idx, ch)) {
                if hint == 'G' {
                    vocab.preserve(words)
                } else {
                    vocab.remove(words)
                }
            }
        }
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

    loop {
        // Pick a random word from the vocabulary. This will be the
        // guess for this iteration of the loop.

        let guess = vocab.pick_random();

        println!(
            "My guess: {} (vocabulary: {} words)",
            guess.to_uppercase(),
            vocab.total()
        );

        // Get hints from the user.

        let input = get_hints()?;

        // If it's GGGGG, then the guess was correct.

        if input == "GGGGG" {
            println!("Solved it! The word was \"{}\"", guess.to_uppercase());
            break;
        }

        // Reduce the vocabulary by applying the hints.

        vocab = process_hints(vocab, &grn_tbl, guess, &input)
    }
    Ok(())
}
