use console::Term;
use console::Key;
use console::style;
use std::time::Instant;

fn main() {
    let term = Term::stdout();

    let target_text = String::from("The quick brown fox jumps over the lazy dog.");
    let mut typed_text = String::new();
    let mut incorrect_counter: usize = 0;
    let mut correct_counter: usize = 0;
    let mut backspace_counter: usize = 0;
    let mut key_timestamps: Vec<Instant> = Vec::new();
    
    let _ = term.clear_screen();
    println!("{}", target_text);
    loop {
        let key_input = term.read_key_raw();
        match key_input {
            Ok(key) => {
                process_input(key, &target_text, &mut typed_text, &mut correct_counter, &mut incorrect_counter, &mut backspace_counter, &mut key_timestamps);

                let _ = term.clear_screen();

                print_current_state(&target_text, &typed_text);

                if target_text.len() == typed_text.len() {
                    print_results(&target_text, correct_counter, incorrect_counter, backspace_counter, key_timestamps);
                    break;
                }
            },
            Err(e) => {
                eprintln!("Error reading key: {}", e);
            }
        }
    }
}

fn process_input(key: console::Key, target_text: &str, typed_text: &mut String, correct_counter: &mut usize, incorrect_counter: &mut usize, backspace_counter: &mut usize, key_timestamps: &mut Vec<Instant>) {
    match key {
        Key::Backspace => {
            typed_text.pop();
            key_timestamps.pop();

            if typed_text.len() != 0 {
                *backspace_counter += 1;
            }
        },
        Key::Char(c) => {
            typed_text.push(c);
            key_timestamps.push(Instant::now());

            if typed_text[typed_text.len() - 1..] == target_text[typed_text.len() - 1..typed_text.len()] {
                *correct_counter += 1;
            } else {
                *incorrect_counter += 1;
            }
        },
        _ => {}
    }
}

fn print_current_state(target_text: &str, typed_text: &str) {
    if typed_text.len() == 0 {
        return;
    }

    let mut color_starts: Vec<usize> = Vec::new();
    let mut currently_correct: bool = typed_text[0..1] == target_text[0..1];

    for (index, char) in typed_text.chars().enumerate() {
        if Some(char) == target_text.chars().nth(index) {
            if !currently_correct {
                color_starts.push(index);
                currently_correct = true;
            }
        } else {
            if currently_correct {
                color_starts.push(index);
                currently_correct = false;
            }
        }
    }

    let mut is_color_correct: bool = typed_text[0..1] == target_text[0..1];
    let mut prev_start: usize = 0;
    for start in color_starts {
        if is_color_correct {
            print!("{}", style(&typed_text[prev_start..start].replace(" ", "•")).green());
        } else {
            print!("{}", style(&typed_text[prev_start..start].replace(" ", "•")).red());
        }
        prev_start = start;
        is_color_correct = !is_color_correct;
    }
    if is_color_correct {
        print!("{}", style(&typed_text[prev_start..].replace(" ", "•")).green());
    } else {
        print!("{}", style(&typed_text[prev_start..].replace(" ", "•")).red());
    }

    println!("{}", &target_text[typed_text.len()..]);
}

fn print_results(target_text: &str, correct_counter: usize, incorrect_counter: usize, backspace_counter: usize, key_timestamps: Vec<Instant>) {
    let total_duration_sec = key_timestamps[key_timestamps.len() - 1]
        .checked_duration_since(key_timestamps[0]).unwrap().as_secs_f32();

    let accuracy = get_accuracy(correct_counter, incorrect_counter);

    println!("Time: {}sec\nAccuracy: {}%\nWPM: {}\nAWPM: {}\nBackspaces: {}",
        total_duration_sec,
        accuracy * 100.0,
        get_wpm(target_text, total_duration_sec),
        get_awpm(target_text, accuracy, total_duration_sec),
        backspace_counter
    );
}

fn get_accuracy(correct_counter: usize, incorrect_counter: usize) -> f32 {
    correct_counter as f32 / (correct_counter + incorrect_counter) as f32
}

fn get_wpm(target_text: &str, total_duration_sec: f32) -> f32 {
    const LETTERS_PER_WORD: f32 = 5.0;
    target_text.len() as f32 / LETTERS_PER_WORD / (total_duration_sec / 60.0)
}

fn get_awpm(target_text: &str, accuracy: f32, total_duration_sec: f32) -> f32 {
    const LETTERS_PER_WORD: f32 = 5.0;
    target_text.len() as f32 / LETTERS_PER_WORD / (total_duration_sec / 60.0) * accuracy
}