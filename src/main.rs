use std::io::{self, Write};

fn calculate_frequency(a4: f64, octave: i32, note: &str) -> f64 {
    let note_semitone: i32 = match note {
        "C" => -9,
        "C#" | "Db" => -8,
        "D" => -7,
        "D#" | "Eb" => -6,
        "E" => -5,
        "F" => -4,
        "F#" | "Gb" => -3,
        "G" => -2,
        "G#" | "Ab" => -1,
        "A" => 0,
        "A#" | "Bb" => 1,
        "B" => 2,
        _ => panic!("Invalid note"),
    };

    // Calculate the frequency
    let semitones_from_a4 = (octave - 4) * 12 + note_semitone;
    a4 * 2.0_f64.powf(semitones_from_a4 as f64 / 12.0)
}

fn user_input_int(prompt: &str) -> i32 {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt is printed
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn user_input_string(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt );
    io::stdout().flush().unwrap(); // Make sure the prompt is printed
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

struct Frequency {
    frequency: f64,
}

impl Frequency {
    fn major_third(&self) -> f64 {
        return self.frequency * 1.25;
    }

    fn minor_third(&self) -> f64 {
        return self.frequency * 1.2;
    }

    fn perfect_fifth(&self) -> f64 {
        return self.frequency * 1.5;
    }

    fn perfect_fourth(&self) -> f64 {
        return self.frequency * 1.3333;
    }

    fn minor_seventh(&self) -> f64 {
        return self.frequency * 1.8;
    }

    fn major_seventh(&self) -> f64 {
        return self.frequency * 1.875;
    }

    fn diminished_fifth(&self) -> f64 {
        return self.frequency * 1.414;
    }

    fn major_second(&self) -> f64 {
        return self.frequency * 1.125;
    }

    fn minor_second(&self) -> f64 {
        return self.frequency * 1.0667;
    }

    fn major_sixth(&self) -> f64 {
        return self.frequency * 1.6667;
    }

    fn minor_sixth(&self) -> f64 {
        return self.frequency * 1.6;
    }
}
fn main() {
    loop {
        let a4 = 440.0;

        // Get octave and note from user input
        let note: String = user_input_string("Enter a note (C, C#, D, D#, E, F, F#, G, G#, A, A#, B): ").to_uppercase();
        let octave: i32 = user_input_int("Enter an octave (-4, -3, -2, -1, 0, 1, 2, 3, 4): ");

        let frequency = calculate_frequency(a4, octave, &note);
        println!("The frequency of {}{} is {:.2} Hz", note, octave, frequency);
        println!(
            "The minor second of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.minor_second()
        );
        println!(
            "The major second of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.major_second()
        );
        println!(
            "The major third of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.major_third()
        );
        println!(
            "The minor third of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.minor_third()
        );
        println!(
            "The perfect fourth of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.perfect_fourth()
        );
        println!(
            "The perfect fifth of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.perfect_fifth()
        );
        println!(
            "The diminished fifth of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.diminished_fifth()
        );
        println!(
            "The major sixth of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.major_sixth()
        );
        println!(
            "The minor sixth of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.minor_sixth()
        );
        println!(
            "The minor seventh of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.minor_seventh()
        );
        println!(
            "The major seventh of {}{} is {:.2} Hz",
            note,
            octave,
            Frequency { frequency }.major_seventh()
        );
        let continue_choice = user_input_string("Do you want to calculate another frequency? (y/n): ");
        if continue_choice.to_lowercase() != "y" {
            break;
        }
    }
}
