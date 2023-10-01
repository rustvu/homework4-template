use rand::prelude::*;
// TODO: bring HashMap into scope

#[derive(Debug, Clone)]
struct MarkovModel(HashMap<(String, String), Vec<String>>);

impl MarkovModel {
    fn new() -> Self {
        MarkovModel(HashMap::new())
    }

    /// Train the model on the given text.
    /// The text is split into words by whitespace.
    /// For each 3 consecutive words in the text, the first 2 words are used as
    /// a key in the hashmap, and the 3rd word is added to the list (value in the hashmap) 
    /// of possible words that can follow the first 2 words.
    /// If there are less than 3 words, the function does nothing (no panic!)
    fn train(&mut self, text: &str) {
        // TODO: implement this function
    }

    fn train_from_file(&mut self, filename: &str) {
        let text = std::fs::read_to_string(filename).unwrap();
        self.train(&text);
    }

    /// Generate text from the model, starting with the given seed.
    /// The seed is split into words by whitespace. The last 2 words are used
    /// as the starting point for the generated text. You can assume that
    /// the seed contains at least 2 words (you can panic otherwise)
    /// The generated text is at most `max_word_count` words long.
    /// If the model does not contain information about the last 2 words at 
    /// any point, the generation stops early.
    /// The generated text (including the seed part) is returned as a single 
    /// string, with words separated by space characters.
    fn generate(&self, seed: &str, max_word_count: i32) -> String {
        // TODO: implement this function
        // Hint: you can pick a random element from a vec or slice using the
        // `choose` method:
        // let mut rng = rand::thread_rng();
        // let v = vec![1,2,3,4];
        // let x = v.choose(&mut rng).unwrap();
    }
}

fn main() {
    let mut model = MarkovModel::new();
    model.train_from_file("sherlock.txt");
    let text = model.generate("Sherlock Holmes", 100);
    println!("{}", text);
}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train() {
        let mut model = MarkovModel::new();
        model.train("a b c d e f a b a b b c a f f f f");
        assert_eq!(model.0.len(), 11);

        assert_eq!(
            model.0.get(&("a".to_owned(), "b".to_owned())),
            Some(&vec!["c".to_owned(), "a".to_owned(), "b".to_owned()])
        );
        assert_eq!(
            model.0.get(&("b".to_owned(), "c".to_owned())),
            Some(&vec!["d".to_owned(), "a".to_owned()])
        );
        assert_eq!(
            model.0.get(&("f".to_owned(), "f".to_owned())),
            Some(&vec!["f".to_owned(), "f".to_owned()])
        );
        assert_eq!(model.0.get(&("y".to_owned(), "z".to_owned())), None);
    }

    #[test]
    fn test_train_short() {
        // Test that training with less than 3 words does not panic and
        // creates no entries.
        let mut model = MarkovModel::new();
        model.train("");
        assert_eq!(model.0.len(), 0);

        model = MarkovModel::new();
        model.train("a");
        assert_eq!(model.0.len(), 0);

        model = MarkovModel::new();
        model.train("a b");
        assert_eq!(model.0.len(), 0);
    }

    #[test]
    fn test_generate() {
        // test deterministic generation (no choices)
        let mut model = MarkovModel::new();
        model.train("a b c d e f");
        let text = model.generate("a b", 100);
        assert_eq!(text, "a b c d e f");

        // test that generation stops at max_word_count
        model = MarkovModel::new();
        model.train("a b c d e f");
        let text = model.generate("a b", 4);
        assert_eq!(text, "a b c d");

        // test that the generated text is not always the same
        model = MarkovModel::new();
        model.train_from_file("sherlock.txt");

        let first_text = model.generate("Sherlock Holmes", 50);
        let same = (0..10).all(|_| {
            let text = model.generate("Sherlock Holmes", 50);
            text == first_text
        });
        assert!(!same, "generated text is always the same");
    }
}
