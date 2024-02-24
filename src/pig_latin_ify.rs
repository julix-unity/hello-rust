/*
 * Convert strings to pig latin. 
 * The first consonant of each word is moved to the end of the word and “ay” is added, 
 * so “first” becomes “irst-fay.” 
 * Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
 * Keep in mind the details about UTF-8 encoding!
*/

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn pig_latin_ify(word: String) -> String {
    let mut first_letter: Option<char> = None;
    let mut new_word = String::new();

    // go over the letters
    for (i, char) in word.chars().enumerate() {
        if i == 0 {
            // store the first letter
            first_letter = Some(char);

            // if it's a vowel
            if VOWELS.contains(&char) {
                // Directly return the result
                return format!("{}-hay", word); 
            }
        } else {
            // store rest of characters
            new_word.push(char);
        }
    }

    // consonant case
    if let Some(first) = first_letter {
        format!("{}-{}ay", new_word, first)

    // empty case
    } else {
        // word is empty, return as is.
        word
    }
}
