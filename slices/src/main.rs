fn main() {
    let s = String::from("hello world");
    let first_word_index = get_first_word_index(&s);

    let first_word = get_first_word(&s);
    println!("First word: {}", first_word);

    println!("First word: {}", &s[0..first_word_index]);

    let (start, end) = get_n_word_index(&s, 2);
    println!("Second word: {}", &s[start..end]);
    let second_word = get_n_word(&s, 2);
    println!("Second word: {}", second_word);
}

fn get_first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_n_word_index(s: &String, n: usize) -> (usize, usize) {
    if n == 0 {
        return (s.len(), s.len());
    }

    let bytes = s.as_bytes();
    let mut word_count = 0;
    let mut in_word = false; // Track if currently inside a word
    let mut start_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if in_word { // end of a word
                if word_count == n {
                    return (start_index, i);
                }
                in_word = false;
            }
        } else if !in_word { // start of a new word
            in_word = true;
            word_count += 1;
            start_index = i;
        }
    }

    // If we reach here, it means we didn't find the nth word within the string
    if in_word && word_count == n {
        return (start_index, s.len());
    }
    
    (s.len(), s.len())
}

//
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn get_n_word(s: &String, n: usize) -> &str {
    if n == 0 {
        return &s[..];
    }

    let bytes = s.as_bytes();
    let mut word_count = 0;
    let mut in_word = false; // Track if currently inside a word
    let mut start_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if in_word { // end of a word
                if word_count == n {
                    return &s[start_index..i];
                }
                in_word = false;
            }
        } else if !in_word { // start of a new word
            in_word = true;
            word_count += 1;
            start_index = i;
        }
    }

    // If we reach here, it means we didn't find the nth word within the string
    if in_word && word_count == n {
        return &s[start_index..s.len()];
    }
    
    &s[..]
}