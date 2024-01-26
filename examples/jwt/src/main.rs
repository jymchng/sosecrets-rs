use dotenvy::dotenv;
use lib::{login, new_token};
use rpassword::prompt_password;
use sosecrets_rs::prelude::Secret;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    println!("Welcome to `sosecrets-rs` example!");

    println!("Please enter your username: ");

    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let token = new_token(
        &username,
        Secret::new(prompt_password("Please enter your password: ")?),
    )?;

    let logged_in_user = login(&*token)?;

    assert_eq!(logged_in_user, "Michael Yang\n");
    println!("Authenticated!");
    println!("Your token is {token}");

    Ok(())
}
