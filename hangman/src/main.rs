use std::{io::{self, Write}};

fn main() {
    let word: &str = "dolphin";
    let menu_choice: bool = menu();
    
    clear_console();

    while menu_choice
    {
        let result =  hangman(word);        
        if result {
            println!("You won!");
            break;
        } else {
            println!("You lost!");
            break;
        }
    } 
}

//Main game
fn hangman(word: &str) -> bool{
    
    let word_vec: Vec<char> = word.chars().collect();
    let word_len = word.len();

    let mut guess_vec: Vec<char> = Vec::new();
    let mut view_vec:Vec<char> = Vec::new();
    for _x in 0..word_len  {
        view_vec.push('_');
    }

    for _x in 0..6 {
        guess_vec.push('_');
    }

    let mut correct_guesses_vec: Vec<char> = Vec::new();
    let mut correct_guesses = 0;
    let mut lives: u8 = 6;

    //Fixed printing by using flush
    while lives >= 1 {
        
        clear_console();

        if correct_guesses == word_len {
            print!("Word is: ");
            io::stdout().flush().unwrap();
            for c in view_vec.iter() {
                print!("{}  ", c);
                io::stdout().flush().unwrap();
            }
            println!();
            break;
        }

        println!("Number of lives: {}", lives);
        io::stdout().flush().unwrap();

        print!("Word is: ");
        io::stdout().flush().unwrap();
        for c in view_vec.iter() {
            print!("{}  ", c);
            io::stdout().flush().unwrap();
        }
        println!();
        io::stdout().flush().unwrap();

        print!("Guesses made: ");
        io::stdout().flush().unwrap();
        for c in guess_vec.iter() {
            print!("{}  ", c);
            io::stdout().flush().unwrap();
        } 
        println!();

        //Input 
        let c = char_input();
        
        let mut found_char = false;

        //Correct guess
        for (i,current_char) in word_vec.iter().enumerate() {
            let mut already_found = false;
            for correct_char in correct_guesses_vec.iter() {
                if &c == correct_char {
                    already_found = true;
                    found_char = true;
                    break;
                }
            }
            if &c == current_char && !already_found
            {
                if let Some(elem) = view_vec.get_mut(i) {
                    *elem = c;
                    correct_guesses += 1;
                    correct_guesses_vec.push(c);
                    found_char = true;
                }
            } 
        }

        //Incorrect guess
        if found_char == false {
            for current_char in guess_vec.iter_mut() {
                if *current_char == c {
                    break;
                }
                if *current_char == '_'{
                    *current_char = c;
                    lives -= 1;
                    break;
                }
            }
        }
    }

    if lives == 0 {
        return false;
    } else {
        return true;
    }

}


//Clear console 
//Bug: Shows up as weird characters in CMD (Windows 10)
//StackOverflow: https://stackoverflow.com/a/66911945
fn clear_console() {
    print!("{esc}c", esc = 27 as char);    
}

//Guessing
fn char_input() -> char {
    
    loop {
        let mut input = String::new();
        
        println!();
        io::stdout().flush().unwrap();
        print!("Guess a letter: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().len() == 1 {
            let c: char = input.chars().next().unwrap();
            if c.is_alphabetic() {
                return c;
            }
        } 
    }
}

//Menu
fn menu() -> bool {
    let option;
    loop {
        println!("Hangman");
        println!("-------");
        println!("Play - p");
        println!("Quit - q");
        println!("-------");
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq("q") {
            option = false;
            break;
        } else if input.trim().eq("p") {
            option = true;
            break;
        }
    }
    return option;

}
