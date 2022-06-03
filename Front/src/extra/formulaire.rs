use argon2::Config;
use passwords::{analyzer, hasher, scorer};
use rocket::form::Form;

#[derive(FromForm)]
pub struct Password {
    #[field(name = "code",validate=len(10..12))]
    code: String,
    #[field(name = "password",validate=len(8..64))]
    password: String,
    #[field(name = "secret",validate=len(8..64))]
    secret: String,
}
impl Password {
    pub fn password(&self) -> ResultForm {
        password_give(self.password.to_string())
    }
    pub fn code(&self) -> Result<String, ()> {
        Ok(self.code.to_string())
    }
    pub fn secret(&self) -> Result<String, ()> {
        Ok(self.secret.to_string())
    }
}
pub type PasswordForm = Form<Password>;
pub type ErrForm = ();
pub type ResultForm = Result<String, ErrForm>;
///
///  Verifie the password and return the hash password or an error 
/// 
fn password_give(password: String) -> ResultForm {
    if scorer::score(&analyzer::analyze(password.to_string())) > 80f64 {
        match argon2::hash_encoded(password.as_bytes(), &hasher::gen_salt(), &Config::default()) {
            Ok(value) => Ok(value),
            Err(_) => Err(()),
        }
    } else {
        Err(())
    }
}
