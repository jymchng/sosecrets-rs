# sosecrets-rs Example on Using `Secret<T, MEC, EC>`

Example is taken and tweaked from [here](https://github.com/mikkyang/rust-jwt/blob/master/examples/custom_claims.rs).

This is an example Rust binary using the `sosecrets-rs` crate to manage secrets, specifically focusing on JWT-based authentication. The example includes two files: `main.rs` and `lib.rs`.

# Usage

Run `cargo run`.

Positive Example Output:

```bash
Welcome to `sosecrets-rs` example!
Please enter your username:
Michael Yang
Please enter your password: // enter literal 'password' here
Authenticated!
```

Negative Example Output:

```bash
Welcome to `sosecrets-rs` example!
Please enter your username:
Michael Yang
Please enter your password: // enter any other literal
Error: "Wrong password used!"
```

# Discussions

1. Composable Exposure of Secrets

Consider:

```rust
pub fn new_token(
    user_id: &str,
    secret_password: Secret<String, U1, U0>,
) -> Result<Secret<String, U1, U0>, Box<dyn Error>> {
    let header: Header = Default::default();
    let claims = Custom {
        sub: user_id.into(),
        rhino: true,
    };
    let unsigned_token = Token::new(header, claims);

    let secret_key = get_secret_key_from_env();

    let (_, signed_token) = secret_key?.expose_secret(move |exposed_key| {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(&*exposed_key.as_bytes()).map_err(|_e| "Invalid key")?;
        let (_, returned_value) = secret_password.expose_secret(|exposed_password| {
            if *exposed_password != "password" {
                return Err("Wrong password used!");
            }
            return Ok(());
        });
        let _ = returned_value?;
        let signed_token = unsigned_token
            .sign_with_key(&key)
            .map_err(|_e| "Sign error")?;
        Ok::<std::string::String, Box<dyn Error>>(signed_token.into())
    });
    Ok::<Secret<String, U1, U0>, Box<dyn Error>>(Secret::new(signed_token?))
}
```

No secrets can be logged during the exposure of `secret_key` (through `exposed_key`) and `secret_password` (through `exposed_password`).

Callers can be assured at API that use `Secret<T, MEC, EC>` can only use it at very specific scope that is tied to an invariant lifetime.