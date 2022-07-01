# A hangman game written in rust

This was just a little intro to rust as I haven't written anything in it yet and I felt that I should get something small out of the way to start.
A lot of the code was taken from this repository https://github.com/katecpp/Hangman. This project was made solely for learning, I took what katecpp implemented in their program and implemented into my own, that is all.

A lot of stuff was unusable in katecpp's repository due to changes in rust, and they used many deprecated features. Many of which I had to replace with their newer versions so that the code would compile correctly. I will most likely build upon this at some other time to actually build upon this instead of just leeching off of it but for now this is what it's going to be.

ALSO this code probably won't work in linux as the clear function is meant for windows consoles and at the end of every game it uses a windows command called 'pause' which I don't believe is supported on linux. Might make it cross-compatible at some point.
