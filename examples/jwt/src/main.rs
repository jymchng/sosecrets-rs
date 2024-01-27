use dotenvy::dotenv;
use lib::{login, new_token};
use rpassword::prompt_password;
use sosecrets_rs::{prelude::Secret, traits::ExposeSecret};
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    println!("Welcome to `sosecrets-rs` example!");

    println!("Please enter your username: ");

    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let secret_token = new_token(
        &username,
        Secret::new(prompt_password("Please enter your password: ")?),
    )?;

    let (_, logged_in_user) =
        secret_token.expose_secret(|exposed_secret_token| login(&*exposed_secret_token));

    assert_eq!(logged_in_user?, "Michael Yang\n");
    println!("Authenticated!");
    // println!("`secret_token` = {secret_token:?}"); // Cannot print it out because
    // `Secret<String, UInt<UTerm, B1>>` cannot be formatted using `{:?}` because it doesn't implement `Debug`

    Ok(())
}
