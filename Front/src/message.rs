use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
enum Type{
    Unknow=0,
    BadFormatPassword=2,
    BadFormatEmail=3,
    OkRemoveUser=5,
    OkAddUser=6,
    BadRemoveNeverExiste=9,
    BadUserYetExiste=8,
    BadPasswordNotMatch=11,
    BadUserNotMatch=12,
    SessionNotFound=13,
    NotCookieFound=14,
}

///
/// message de debug pour chaque situation (à compléter)
/// 
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    code: Type,
    message: &'static str,
    extra: String
}
impl Message {
    pub fn unknow() -> Message {
        Message {
            code: Type::Unknow,
            message: "the ressource have a problem",
            extra:String::new()
        }
    }
    pub fn bad_email_format() -> Message {
        Message {
            code: Type::BadFormatEmail,
            message: "The target 'email' have nt a good email format",
            extra:String::new()
        }
    }
    pub fn bad_password_format() -> Message {
        Message {
            code: Type::BadFormatPassword,
            message: "The target 'password' have nt a good password format",
            extra:String::new()
        }
    }
    pub fn ok_user_added() -> Message {
        Message {
            code: Type::OkAddUser,
            message: "The user is add to the database",
            extra:String::new()
        }
    }
    pub fn ok_user_remove() -> Message {
        Message {
            code: Type::OkRemoveUser,
            message: "The user is remove from the database",
            extra:String::new()
        }
    }
    pub fn bad_user_yet_existe() -> Message {
        Message {
            code: Type::BadUserYetExiste,
            message: "The user is already in the database",
            extra:String::new()
        } 
    }
    pub fn bad_remove_user_never_existe() -> Message {
        Message {
            code: Type::BadRemoveNeverExiste,
            message: "The user is not in the database",
            extra:String::new()
        } 
    }
    pub fn password_not_math() -> Message {
        Message {
            code: Type::BadPasswordNotMatch,
            message: "The password not match",
            extra:String::new()
        } 
    }
    pub fn user_not_match() -> Message {
        Message {
            code: Type::BadUserNotMatch,
            message: "The user not match",
            extra:String::new()
        } 
    }
    pub fn session_not_found() -> Message {
        Message {
            code: Type::SessionNotFound,
            message: "The user not match",
            extra:String::new()
        } 
    }
    pub fn no_cookie() -> Message {
        Message {
            code: Type::NotCookieFound,
            message: "The cookie is not found",
            extra:String::new()
        } 
    }
    pub fn json(self) -> MessageJson {
        Json(self)
    }
    pub fn set_extra(mut self,extra:String) -> Self{
        self.extra=extra;
        self
    }
}
pub type MessageJson = Json<Message>;