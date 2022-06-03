use argon2::Config;
use passwords::{analyzer, hasher, scorer};
use regex::Regex;
use rocket::form::Form;
use super::super::message::Message;
#[derive(FromForm)]
pub struct Connect {
    #[field(name = "password",validate=len(8..64))]
    password: String,
    #[field(name = "email",validate=len(4..64))]
    email: String,
    #[field(name = "code",validate=len(..7))]
    code: Option<String>,
}
impl Connect {
    pub fn password(&self) -> Result<String, Message> {
        Ok(self.password.to_string())
    }
    pub fn code(&self) -> Result<String, Message> {
        Ok(self.code.clone().unwrap_or_default())
    }
    pub fn email(&self) -> Result<String, Message> {
        if email_validator(self.email.clone()) {
            Ok(self.email.clone())
        } else {
            Err(Message::bad_email_format())
        }
    }
}
pub type ConnectForm = Form<Connect>;

#[derive(FromForm)]
pub struct UpdatePassword {
    #[field(name = "new_password",validate=len(8..64))]
    new_password: String,
    #[field(name = "password",validate=len(8..64))]
    password: String,
}
impl UpdatePassword {
    pub fn password(&self) -> Result<String, Message> {
        Ok(self.password.to_string())
    }
    pub fn new_password(&self) -> Result<String, Message> {
        password_give(self.new_password.to_string())
    }
}
pub type UpdatePasswordForm = Form<UpdatePassword>;

#[derive(FromForm)]
pub struct CreateUser {
    #[field(name = "email",validate=len(4..64))]
    email: String,
    #[field(name = "first_name",validate=len(2..64))]
    firstname: String,
    #[field(name = "last_name",validate=len(2..64))]
    lastname: String,
}

impl CreateUser {
    pub fn email(&self) -> Result<String, Message> {
        if email_validator(self.email.clone()) {
            Ok(self.email.clone())
        } else {
            Err(Message::bad_email_format())
        }
    }
    pub fn firstname(&self) -> Result<String, Message> {
        Ok(self.firstname.clone())
    }
    pub fn lastname(&self) -> Result<String, Message> {
        Ok(self.lastname.clone())
    }
}
pub type CreateUserForm = Form<CreateUser>;

#[derive(FromForm)]
pub struct RemoveUser {
    #[field(name = "email", validate=len(4..64))]
    email: String,
}

impl RemoveUser {
    pub fn email(&self) -> Result<String, Message> {
        if email_validator(self.email.clone()) {
            Ok(self.email.clone())
        } else {
            Err(Message::bad_email_format())
        }
    }
}
pub type RemoveUserForm = Form<RemoveUser>;

///
///  Verifie the password and return the hash password or an error 
/// 
fn password_give(password: String) -> Result<String, Message> {
    if scorer::score(&analyzer::analyze(password.to_string())) > 80f64 {
        match argon2::hash_encoded(password.as_bytes(), &hasher::gen_salt(), &Config::default()) {
            Ok(value) => Ok(value),
            Err(_) => Err(Message::unknow()),
        }
    } else {
        Err(Message::bad_password_format())
    }
}

///
/// boolean de validation du format de l'email
/// 
fn email_validator(email: String) -> bool {
    Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap()
    .is_match(email.as_str())
}
