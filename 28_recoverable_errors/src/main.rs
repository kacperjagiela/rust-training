use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_resut = File::open("hello.txt");

    let greeting_file = match greeting_file_resut {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let greeting_file =
        File::open("hello1.txt").expect("hello.txt should be included in this project");

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello1.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_shorter() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;

        Ok(username)
    }

    fn read_username_from_file_shorter2() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username);

        Ok(username)
    }

    fn read_username_from_file_shorter3() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
