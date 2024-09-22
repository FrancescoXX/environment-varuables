/* Environment Variables 
They are a set of key-value pairs 
that are stored in the operating system 
and are used to store configuration settings 
and other information that is used by the system 
and other applications. */

use std::env; // Import the env module
use dotenv::dotenv;

fn main() {
    let key = "AAA";
    std::env::set_var(key, "123"); //AAA,123

    //remove the key
    // env::remove_var(key);

    // use a match statement to check if the key exists
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!("Error {}: {}", key, e),
    }

    // read env variables from the command line
    //CLI_ARG=TEST cargo run
    //$env:CLI_ARG="TEST"; cargo run
    let cli_arg = env::var("CLI_ARG");

    match cli_arg {
        Ok(val) => println!("CLI_ARG: {:?}", val),
        Err(e) => println!("Error CLI_ARG: {}", e),
    }

    //Read the environment variables from a file
    dotenv().ok(); // Read the .env file

    let api_key = env::var("API_KEY");

    // println!("Value of API_KEY: {:?}", api_key.expect("API_KEY is not set"));

    match api_key {
        Ok(val) => println!("API_KEY: {:?}", val),
        Err(e) => println!("Error API_KEY: {}", e),
    }

    println!("Hello, world!");
}
