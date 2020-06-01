fn main() {
    let string = String::from("carson is my name");
    println!("{}", find_first_word(&string));
    println!("{}", find_second_word(&string));
}

fn find_first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // i is the index in the tuple and &item is the byte in the tuple
        if item == b' ' { // check if the character is a space
            return &s[0..i]; // return the slice of the beginning of the string to the first space
        }
    }

    &s[..]
}

fn find_second_word(input_string: &String) -> &str {
    let bytes = input_string.as_bytes();
    let mut space_count = 0;
    let mut first_space_index = 0;

    for (i, &current_char) in bytes.iter().enumerate() {
        if current_char == b' ' {
            space_count += 1;

            if space_count == 1 {
                first_space_index = i;
            } else if space_count == 2 {
                // let mut second_space_index = i;
                return &input_string[first_space_index..i];
            }
        }
    }

    &input_string
}

