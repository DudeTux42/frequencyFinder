# frequencyFinder
This is a small programm written in Rust to convert notes to the matching frequency.

# Documentation for Musical Frequency Calculator

## Overview
This program calculates the frequency of a musical note based on a given note name and octave, starting from the A4 reference pitch (440 Hz). It also provides the frequencies of intervals relative to the calculated frequency, including major/minor seconds, thirds, fourths, fifths, sixths, and sevenths.

## Functions and Structs

### `calculate_frequency(a4: f64, octave: i32, note: &str) -> f64`
Calculates the frequency of a given musical note in a specified octave.
- **Parameters**:
  - `a4`: The reference frequency for the A4 note (usually 440 Hz).
  - `octave`: The octave in which the note resides. Octave 4 is the reference for A4.
  - `note`: The musical note (C, C#, D, D#, E, F, F#, G, G#, A, A#, B). Accepts sharps (`#`) and flats (`b`).
- **Returns**: The frequency of the note as a floating-point value.

### `user_input_int(prompt: &str) -> i32`
Prompts the user for an integer input.
- **Parameters**:
  - `prompt`: A string to display as a prompt to the user.
- **Returns**: The user’s input as an integer.

### `user_input_string(prompt: &str) -> String`
Prompts the user for a string input.
- **Parameters**:
  - `prompt`: A string to display as a prompt to the user.
- **Returns**: The user’s input as a string.

### `struct Frequency`
This struct stores the frequency of a note and provides methods to calculate the frequency of various musical intervals.
- **Fields**:
  - `frequency`: The frequency of the note (in Hz).

#### Methods
- **`major_third(&self) -> f64`**: Returns the frequency of the major third interval.
- **`minor_third(&self) -> f64`**: Returns the frequency of the minor third interval.
- **`perfect_fourth(&self) -> f64`**: Returns the frequency of the perfect fourth interval.
- **`perfect_fifth(&self) -> f64`**: Returns the frequency of the perfect fifth interval.
- **`minor_seventh(&self) -> f64`**: Returns the frequency of the minor seventh interval.
- **`major_seventh(&self) -> f64`**: Returns the frequency of the major seventh interval.
- **`diminished_fifth(&self) -> f64`**: Returns the frequency of the diminished fifth interval.
- **`major_second(&self) -> f64`**: Returns the frequency of the major second interval.
- **`minor_second(&self) -> f64`**: Returns the frequency of the minor second interval.
- **`major_sixth(&self) -> f64`**: Returns the frequency of the major sixth interval.
- **`minor_sixth(&self) -> f64`**: Returns the frequency of the minor sixth interval.

### `fn main()`
The main function runs an interactive loop where the user can:
1. Input a musical note and octave.
2. Calculate the frequency of the note.
3. Display the frequency of the note along with various musical intervals.
4. Optionally, repeat the process for another note.

- **Loop process**:
  - Prompts the user for a note and octave.
  - Computes and displays the note's frequency.
  - Computes and displays the frequencies of musical intervals relative to the note.
  - Asks the user if they wish to calculate another note. The loop exits if the user inputs 'n' or anything other than 'y'.

### Example Output:
```
Enter a note (C, C#, D, D#, E, F, F#, G, G#, A, A#, B): C
Enter an octave (-4, -3, -2, -1, 0, 1, 2, 3, 4): 4
The frequency of C4 is 261.63 Hz
The minor second of C4 is 279.00 Hz
The major second of C4 is 294.33 Hz
The major third of C4 is 327.04 Hz
The minor third of C4 is 313.96 Hz
The perfect fourth of C4 is 348.85 Hz
The perfect fifth of C4 is 392.44 Hz
The diminished fifth of C4 is 370.12 Hz
The major sixth of C4 is 436.41 Hz
The minor sixth of C4 is 418.61 Hz
The minor seventh of C4 is 471.34 Hz
The major seventh of C4 is 490.57 Hz
Do you want to calculate another frequency? (y/n): n
```

## Error Handling
- The program will panic if an invalid note is entered.
- Integer and string inputs are validated, with appropriate prompts for incorrect input.

## Usage
This program is useful for musicians or developers working with musical frequencies, and it provides an interactive way to explore the relationships between notes and intervals.

# ToDo's
Icon, 
Maybe an Egui interface
