fn main() {
    let string = String::from("carson is my name");
    let first_word = find_nth_word(&string, 1);
    let second_word = find_nth_word(&string, 2);
    println!("{}", first_word);
    println!("{}", second_word);
}

// Finds nth word in a string and returns it as a string
fn find_nth_word(input_string: &str, n: i32) -> &str {
    let mut space_count = 0; // count which word we're on by counting spaces
    let mut first_space_index = 0;

    for (i, &current_char) in input_string.as_bytes().iter().enumerate() {
        if current_char == b' ' { // check if character is a space
            space_count += 1;

            if space_count == n - 1 {
                first_space_index = i + 1;
            } else if space_count == n {
                return &input_string[first_space_index..i];
            }
        }
    }

    &input_string // Return entire string as default
}
