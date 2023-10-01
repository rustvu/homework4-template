# RustVU (CS 3891) - Homework 4

## Optional / warmup exercises for the week

I highly recommend the [Rustlings project](https://github.com/rust-lang/rustlings) for practicing the basic concepts we learn in this class. These completely optional, not graded/submitted exercises can help you to teach Rust programming to your "fingers".

The recommended exercises for this week:

- `strings`
- `vecs`
- `hashmaps`

## Assignment

This assignment contains a single Rust crate focusing on vectors and hashmaps.

The program (`markov` crate) is supposed to implement a [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) text generator, one of the simplest probabilistic approach for generating text by learning some statistical patterns in natural language samples.

The basic idea is to build a statistical database during the training process which stores the potential next word based on the previous N (N=2 for the homework) words in the sample text. The theoretical Markov model describes the probability of the next word based on the previous N words (the current state), but it is easier to implement this by storing all next word occurrences (even the same word multiple times, if this is encountered multiple times in the sample text). During the text generation phase (after learning from the sample text) we can use a simple random choice from this list. If the same word is encountered multiple times, this approach will result in a higher chance to pick that word in the generation phase. Thus you do not need to deal with actual probability values explicitly.

The statistical database is stored as a hashmap, where the keys are tuples of two strings (the previous two words) and the values are list of strings: all the words (the same word even multiple times - see above) encountered in the sample text after these two words.

Your tasks are the implement the learning and the text generation algorithms. For the generation task you will be given a text fragment with two starting words and you should use your statistical database to generate a longer text by randomly picking next words iteratively until you reach a maximum word limit or you need to stop early because you do not have any information in the database on the last two words in the generated text.

The main function uses a sample text ([The Adventures of Sherlock Holmes by Arthur Conan Doyle](https://www.gutenberg.org/ebooks/1661)) from [Project Gutenberg](https://www.gutenberg.org/) to demonstrate the process. Have fun with it.

Make sure that crate compiles and builds properly and passes all of the built-in tests.

I placed `// TODO` comments in the code where I expect you to add implementation code. The test code is clearly marked with a `// DO NOT EDIT BELOW THIS LINE` comment. This should be evident: changing the test code is a (not too intelligent) way of cheating. I will handle any such attempts accordingly. However, you are allowed and encouraged to look at the test code to better understand what is expected from you.

## Use

You can always check your work with `cargo test`. You can also run individual tests by running `cargo test <test-name>` (see the names below).

The crate contains a sample `main()` function (see comments above). You can modify this function for development/debugging purposes. The output of `cargo run` is not tested or graded.

## Grading

Make sure you __commit__ and __push__ your assignment repository once you manage to run `cargo test` without any errors or warnings.

The homework is graded by test (no partial credits are given for failed tests):

|Test          |Points|
|--------------|------|
|test_train    |    60|
|test_generate |    40|

Once you __push__ your solution to the repository, GitHub Classroom will run the automated test. I highly recommend to [verify your results of this CI/CD worflow](https://docs.github.com/en/education/manage-coursework-with-github-classroom/learn-with-github-classroom/view-autograding-results) - I use these results for grading your work.
