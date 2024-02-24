

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

    for (i, char) in word.chars().enumerate() {
        
        // first letter is vowel case
        if i == 0 && VOWELS.contains(&char) {
            // pig-latin-ify and exit
            new_word = format!("{word}-hay");
            break;
        }
        
        // first letter is consonant case
        if i == 0 {
            // store first letter to append at the end
            println!("test");
            first_letter = Some(char);
        } else if i != word.len() - 1 {
            // copy main letters to new word
            new_word.push(char);
        } else {
            // last letter, handle like main letters and pig-latin-ify
            new_word.push(char);
            new_word.push('-');
            if let Some(first) = first_letter {
                new_word.push(first);
            }
            new_word.push('a');
            new_word.push('y');
        }
    }
    return new_word;
}
