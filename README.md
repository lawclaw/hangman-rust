# hangman-rust
A very scuffed attempt at making the Hangman game in as a command-line application in Rust

## Background
Following my first year at university, after I learned the foundations of C and C++, I got introduced to Rust.   
I knew bits and pieces of pointer arithmetic, referencing/dereferencing along with memory management.  
Rust for me was sold as memory safe language and I wanted to try learning a new language on my own.  
(Also the fact that it was a very beloved language on StackOverflow)  
I found many parallels to C and C++ along with other languages such as Haskell.   
Rust in my opinion was like a mixture of multiple languages and it was interesting how multiple features from other languages had been implemented.  
- Stating the return type for a function at the end of the definition (Haskell)
- The use of stdin, stdout etc. (C, C++)
- Much more low-level than I expected
- Referencing and pointers (C, C++)

## Process
> *Syntactically the most different from any other language I have coded in*

My experiences so far has been in mainly statically typed languages, the syntax has been the most different and difficult to learn.  
Most of the time I don't have to specify the datatype for my variables which was something that had been engrained in me.   
For the most part it made it less tedious to type but in other cases I became confused when trying to implement capabilities like comparison/equality capability.  

As for overall program architecture I tried to split most of the key methods:
- main(): console/main window
- hangman(): "game engine"
- clear_console(): clears the console (thank you StackOverflow) - does not properly work in Windows CMD
- char_input: checks the input for guesses
- menu(): game menu

I have yet to implement a OOP approach as I have not come so far in learning.  

Other features that I struggled with was the very strict but informative error and warning messages.  
(Also it really annoyed me that I had to flush the printf() similar to C...)

## Improvements and conclusion
This has been my small project in Rust.  
The only bug is that clear_console() doesn't function properly in CMD (Win 10)  
It has been a challenge and there are many things left for me to learn like:
- The borrowing and ownership system
- Referencing (more in-depth)
- How functional programming can be done
As for improvements to this project would be:
- OOP approach 
- Potential GUI

## Edit 1: Added a file reading function and list of words.  
The game can now be played until the player chooses to end.   
After each game is finished the player will be brought back to the menu where they can choose to play again or quit.  
I have added a noun list which picks a random word every game using rand::Rng.  
The .txt file was found on [this site](http://www.desiquintans.com/nounlist).  
