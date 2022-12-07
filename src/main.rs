use clearscreen;
use rand::distributions::Distribution;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    clearscreen::clear().expect("Clearing screen.");
    loop {
        let choice = menu();
        match choice {
            1 => {
                let new_entry = Entry::builder().site_app().username().password().build();
                Entries::write_new_entry(new_entry);
            }
            2 => {
                let entries = Entries::load_entries();
                let entry = entries::poll_entry();
            }
            3 => {
                let entries = Entries::load_entries();
                entries.show_all();
            }
            4 => {}
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entries {
    list: Vec<Entry>,
}

impl Entries {
    fn new() -> Entries {
        Entries { list: Vec::new() }
    }

    fn show_all(self) {
        for entry in self.list {
            println!("{}", entry);
            println!("");
        }
        pause();
        clearscreen::clear().expect("Clearing screen.");
    }

    fn poll_entry() -> String {
        println!("Which password would you like to retrieve?");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .trim()
            .expect("Failed to read line.");
        answer
    }

    fn load_entries() -> Entries {
        let path = Path::new("./passwords.json");
        let path_exists: bool = path.exists();
        if path_exists == true {
            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
            };
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(contents) => contents,
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
            };
            let entries: Entries =
                serde_json::from_str(&contents).expect("Parsing contents to JSON.");
            entries
        } else {
            let entries = Entries::new();
            entries
        }
    }

    fn write_new_entry(new_entry: Entry) {
        let path = Path::new("./passwords.json");
        let mut entries = Entries::load_entries();
        entries.list.push(new_entry);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        serde_json::to_writer(file, &entries).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    site_app: String,
    username: String,
    password: String,
}

impl Entry {
    fn builder() -> EntryBuilder {
        EntryBuilder::default()
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Site/App: {}\n - Username: {}\n - Password: {}",
            self.site_app, self.username, self.password
        )
    }
}

struct EntryBuilder {
    site_app: String,
    username: String,
    password: String,
}

impl EntryBuilder {
    fn new() -> EntryBuilder {
        EntryBuilder {
            site_app: String::from(""),
            username: String::from(""),
            password: String::from(""),
        }
    }

    fn site_app(&mut self) -> &mut EntryBuilder {
        let mut site_app_name = String::new();
        println!("Enter the name of the site or application:\n");
        std::io::stdin()
            .read_line(&mut site_app_name)
            .expect("Failed to read site or application name.");
        let site_app_name = site_app_name.trim().to_string();
        self.site_app = site_app_name;
        clearscreen::clear().expect("Failed to clear screen at site/application entry.");
        self
    }

    fn username(&mut self) -> &mut EntryBuilder {
        let mut user_name = String::new();
        println!("Enter your username for the site or application:\n");
        std::io::stdin()
            .read_line(&mut user_name)
            .expect("Failed to read username.");
        let user_name = user_name.trim().to_string();
        self.username = user_name;
        clearscreen::clear().expect("Failed to clear screen at username entry.");
        self
    }

    fn password(&mut self) -> &mut EntryBuilder {
        let password_length = get_password_length();
        let pass_word = generate_password(password_length);
        self.password = pass_word;
        self
    }

    fn build(&mut self) -> Entry {
        Entry {
            site_app: self.site_app.to_owned(),
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        }
    }
}

impl Default for EntryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

struct MixedChars;

impl Distribution<char> for MixedChars {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        *b"!1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@#$%^&*(){}:;"
            .choose(rng)
            .unwrap() as char
    }
}

fn get_password_length() -> u8 {
    loop {
        println!("How long do you want your password to be?");
        println!("Enter a number between 12 and 18:\n");

        let mut user_response = String::new();
        std::io::stdin()
            .read_line(&mut user_response)
            .expect("Failed to Read Line.");
        let trimmed_response = user_response.trim();

        let parsed_response_result = trimmed_response.parse::<u8>();
        match parsed_response_result {
            Ok(num) => {
                let acceptable_range: Vec<u8> = (12..19).collect();
                if acceptable_range.contains(&num) {
                    clearscreen::clear().expect("Clearing screen.");
                    return num;
                } else {
                    println!("Password must be between 12 and 18 characters.");
                    user_response.clear();
                }
            }
            Err(_) => {
                println!("Please enter a valid integer.");
                user_response.clear();
            }
        }
    }
}

fn generate_password(password_length: u8) -> String {
    let password: String = rand::thread_rng()
        .sample_iter(&MixedChars)
        .take(password_length.into())
        .map(char::from)
        .collect();
    password
}

fn menu() -> i32 {
    println!("MENU");
    println!("What would you like to do?");
    println!("- Enter 1 to make a password.");
    println!("- Enter 2 to view a password.");
    println!("- Enter 3 to view all passwords.");
    println!("- Enter 4 to change a password.");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Reading line");

    let choice: i32 = choice.trim().parse().expect("Could not convert");
    clearscreen::clear().expect("Clearing screen.");
    choice
}

fn pause() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "Press any key to continue...\r\n").unwrap();
    stdout.flush().unwrap();
    stdin().keys().next();
}
