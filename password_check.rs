use regex::Regex;
use std::io;

// Function to check the strength of a password:
fn check_password_strength(password: &str) -> bool {
    // Define regular expressions for different types of characters:
    let re_lowercase = Regex::new(r"[a-z]").unwrap(); // lowercase letters
    let re_uppercase = Regex::new(r"[A-Z]").unwrap(); // uppercase letters
    let re_number = Regex::new(r"[0-9]").unwrap(); // numbers
    let re_special_char = Regex::new(r#"[!@#$%^&*(),.?":{}|<>]"#).unwrap(); // special characters

    // Check if the password satisfies the defined criteria:
    let is_minimum_length = password.len() >= 25; // minimum length of 25 characters
    let has_lowercase = re_lowercase.is_match(password); // check for lowercase letters
    let has_uppercase = re_uppercase.is_match(password); // check for uppercase letters
    let has_number = re_number.is_match(password); // check for numbers
    let has_special_char = re_special_char.is_match(password); // check for special characters

    // Return true if all criteria are met:
    is_minimum_length && has_lowercase && has_uppercase && has_number && has_special_char
}

fn main() {
    // Prompt the user to enter a password:
    println!("Please enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    let password = password.trim(); // remove leading and trailing whitespaces

    // Check the strength of the entered password and print the result:
    if check_password_strength(&password) {
        println!("The password is strong!");
    } else {
        println!("The password is weak! Please use a stronger password.");
    }
}
