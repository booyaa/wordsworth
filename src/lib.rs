// Copyright 2016 Mark Sta Ana.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

/// Returns the syllable count for a word
pub fn syllable_counter(word: &str) -> u32 {
    const DEBUG: bool = false; // must be a better way to do this

    let vowels = "aeiouy";
    let specials = vec!["ia", "ea"]; // single syllables in words like bread and lead, but split in names like Breanne and Adreann TODO: handle names, where we only use "ia"
    let specials_except_end = vec!["ie", "ya", "es", "ed"]; // seperate syllables unless ending the word

    let mut count = 0;
    let mut last_letter: char = ' ';
    let mut last_was_vowel = false;
    let normalised = word.to_lowercase();

    for letter in normalised.chars() {
        if vowels.chars().any(|c| c == letter) {

            // don't count diphthongs unless special cases
            let mut combo = String::new();
            combo.push(last_letter);
            combo.push(letter);

            let is_not_in_special = !&specials.iter().any(|x| x == &combo);
            let is_not_in_specials_except_end = !&specials_except_end.iter().any(|x| x == &combo);
            if last_was_vowel && is_not_in_special && is_not_in_specials_except_end {
                last_was_vowel = true;
            } else {
                count += 1;
                if DEBUG {
                    println!("count: {} => {}", count, letter);
                }
                last_was_vowel = true;
            }
        } else {
            last_was_vowel = false;
        }
        last_letter = letter;
    }

    if word.len() > 2 {
        let testcase_1 = &word[word.len() - 2..];
        let testcase_2 = &word[word.len() - 1..];


        if DEBUG {
            println!("test1 {} test2 {}", testcase_1, testcase_2);
        }
        if specials_except_end.iter().any(|x| x == &testcase_1) {
            if DEBUG {
                println!("!!{} match test1 in specials_except_end", word);
            }
            count -= 1;
        } else if testcase_1 != "ee" && testcase_2 == "e" && normalised != "the" {
            if DEBUG {
                println!("!!{} test1 and test2", word);
            }
            count -= 1;
        }
    }
    count
}

/// Returns a bool value for a haiku candidate
pub fn is_haiku(poem: &str) -> bool {
    // checks 3 lines
    // lines 1 and 3 = 5, otherwise 7
    let mut syllable_total = 0;
    let mut line_count = 0;

    for line in poem.lines() {
        println!("line: {:?}", line);
        for word in line.split(" ") {
            let syllable_count = syllable_counter(&word);
            syllable_total += syllable_count;
            // println!("\tword: {} - syllables: {}", word, syllable_count);
        }
        line_count += 1;
        // println!("line: {} subtotal: {}", line_count, syllable_total);
    }

    line_count == 3 && syllable_total == 17
}


#[cfg(test)]
#[test]
fn correct_syllable_count() {
    // TODO: Create a hashmap
    assert_eq!(1, syllable_counter("the"));
    assert_eq!(3, syllable_counter("lucozade"));
    assert_eq!(1, syllable_counter("love"));
    assert_eq!(2, syllable_counter("dodo"));
    assert_eq!(1, syllable_counter("world"));
    assert_eq!(2, syllable_counter("atom"));
    assert_eq!(3, syllable_counter("energy"));
    assert_eq!(4, syllable_counter("combination"));
}

// TODO: fn known_trixsy_words
#[test]
fn this_is_a_haiku() {
    // haiku source: https://www.youngwriters.co.uk/types-haiku-poem
    let haiku = "The sky is so blue.\nThe sun is so warm up high.\nI love the summer.";
    assert_eq!(true, is_haiku(&haiku));
}

// TODO: this_is_not_a_haiku - not 5-7-5
// TODO: this_is_not_a_haiku - not 3 lines
