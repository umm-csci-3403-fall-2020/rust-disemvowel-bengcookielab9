use std::env;
use std::fs;
use std::path::Path;

//The following crates are used for testing
extern crate tempfile; //Creates temp files and directories
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command; // Run programs // Used for writing assertions

/*
This method should be called with 2 command line arguments, both file names. The first should be the name of a file
containing the text to disemvowel, and the second should be the file we want to write the disemvowled text to
*/
fn main() {
    let args: Vec<String> = env::args().collect(); //put all command line arguments into a vector
    match args.len() {
        0 | 1 | 2 => panic!("Not enough arguments; exactly 2 arguments needed"), //if too few arguments are passed in, return this
        3 => { //run this when there are exactly 2 arguments passed in
            let input_path = Path::new(&args[1]);
            let s = String::from(read_file(&input_path)); 

            let s_disemvowel = disemvowel(&s);

            //creating a new path with the second argument and writing out the disemvoweled string
            write_file(Path::new(&args[2]), &s_disemvowel); 
        },
        _ => panic!("Too many arguments; exactly 2 arguments needed") //if there are too many arguments passed in, return this
    }

}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Could not read the file")
}
fn write_file(path: &Path, s: &str) {
    fs::write(path, s).expect("Unable to write file");
}

/*
This method will peek at the characters of the input string. It loops through the string using Some(c) which means
as long as there are still some characters to look at look at the next character. If the next character matches a vowel,
nothing is done, so it's essentially skipped. _ => output.push(c) will look at any other character in the input string and
push it to the newly created output string. The disemvoweled string is then returned
*/
fn disemvowel(s: &str) -> String {
    let mut input_chars = s.chars().peekable();
    let mut output = String::new();
    while let Some(c) = input_chars.next() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {} 
            _ => output.push(c) 
        }
    }  
    String::from(output) 
}

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::*;
    mod disemvowel {
        use super::*;
        #[test]
        fn hello_world() {
            let input = "Hello, world!";
            let expected = "Hll, wrld!";

            assert_eq!(expected, disemvowel(input));
        }

        #[test]
        fn empty() {
            assert_eq!("", disemvowel(""));
        }

        #[test]
        fn no_vowels() {
            assert_eq!("pqrst", disemvowel("pqrst"));
        }

        #[test]
        fn all_vowels() {
            assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
        }

        #[test]
        fn morris_minnesota() {
            assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
        }

        #[test]
        fn handle_punctuation() {
            assert_eq!(
                "n (nxplnd) lphnt!",
                disemvowel("An (Unexplained) Elephant!")
            );
        }

        #[test]
        fn handle_unicode() {
            assert_eq!("Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳", disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳"));
            assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺", disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
            assert_eq!(" lttl Phncn: 𐤀	𐤈	𐤔	𐤕", disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕"));
            assert_eq!(
                "W cn hndl mj s wll! 🤣😃👍",
                disemvowel("We can handle emoji as well! 🤣😃👍")
            )
        }
    }

    // Tests that check that the correct panics are generated when
    // there aren't the correct number of command line arguments
    // or the input file isn't readable.
    // Needed to fix the cargo_bin name to "disemvowel-in-rust"
    mod panic_tests {
        use super::*;
        #[test]
        fn requires_two_arguments_under() {
            let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap(); 
            cmd.arg("1");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Not enough arguments; exactly 2 arguments needed"));
        }
        //adding this test to specify the condition when there are too many arguments
        #[test]
        fn requires_two_arguments_over() {
            let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
            cmd.arg("1")
                .arg("2")
                .arg("3");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Too many arguments; exactly 2 arguments needed"));
        }
        #[test]
        fn requires_read_file() {
            let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
            cmd.arg("/this/path/does/not/exist")
                .arg("output/path/doesnt/matter");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Could not read the file"));
        }
    }
}
