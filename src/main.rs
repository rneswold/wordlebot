use std::io::{self, Write};

mod dictionary;

fn get_feedback() -> io::Result<String> {
    loop {
        let mut input = String::new();

        print!("  Result> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_uppercase();

        if input.len() != 5 {
            println!("ERROR: result information must be 5 characters");
            continue;
        }

        if input
            .matches(|c| c == 'B' || c == 'Y' || c == 'G')
            .collect::<Vec<&str>>()
            .len()
            < 5
        {
            println!("ERROR: only letters in result are B, Y, and G");
            continue;
        }

        return Ok(input);
    }
}

fn process_result(
    vocab: dictionary::Words, gt: &dictionary::GreenTable, guess: &str,
    result: &str,
) -> dictionary::Words {
    result
        .char_indices()
        .filter_map(|(idx, ch)| {
            if ch == 'G' {
                let guess_ch = guess.chars().nth(idx).unwrap();

                gt.get(&(idx, guess_ch))
            } else {
                None
            }
        })
        .fold(vocab, dictionary::Words::remove)
}

fn main() -> io::Result<()> {
    let mut vocab = dictionary::get_vocabulary();
    let grn_tbl = dictionary::GreenTable::new();

    loop {
        let guess = vocab.pick_random();

        println!(
            "My guess: {} (vocabulary: {} words)",
            guess.to_uppercase(),
            vocab.total()
        );

        let input = get_feedback()?;

        if input == "GGGGG" {
            println!("Solved it! The word was \"{}\"", guess.to_uppercase());
            break;
        }

        vocab = process_result(vocab, &grn_tbl, &guess, &input)
    }
    Ok(())
}
