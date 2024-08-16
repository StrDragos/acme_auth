use serde::de::Error;

struct AuthService {}

impl AuthService {
    fn link_sign_in(email: &str) -> Result<(), Box<dyn Error>> {
        println!("Received {}", email);
        Ok(())
    }

    fn social_sign_in(token: &str) -> Result<(), Box<dyn Error>> {
        println!("Received token {}", token);
        Ok(())
    }
}
