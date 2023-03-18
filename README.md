# Rust vs TS - Large JSON processing
This project is only aiming to make a personal study and observation on some cases regarding JSON processing (in large amounts), comparing Rust vs Typescript (which is my daily driver). It is also my first touch with the Rust language, so every rust commit is basically me learning a new thing.   

## What it will be measuring?
Well, this is more a test than anything, but nothing actually scientific.  
I have made a ~24MB JSON using the website [json-generator](https://json-generator.com/), containing some like-user data, and each one of these users having N `tags`, I will get each one of these tags and add the specific user to the grouped tags users... basically concatening them on each of one the tags in the array.  
Running a timestamp before it starts running, and another after it all ran, I will get the time it took between one timestamp declaration to another and display the results.

## Results
still developing...

### Notes to self
- reading files with Rust is due to where you are at your terminal when you run `cargo run`... interesting... i think
