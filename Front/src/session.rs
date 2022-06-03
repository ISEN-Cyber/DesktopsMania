use self::diesel::prelude::*;
use super::message::Message;
use super::schema::*;
use diesel::dsl::now;
use diesel::PgConnection as DbConnection;
use google_authenticator::GoogleAuthenticator;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Serializer};
use rocket_sync_db_pools::diesel;

pub type JsonInfo = Json<Session>;

///
/// Gestion des sessions
/// 
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    authority: LEVEL,
    path: String,
    #[serde(skip)]
    cookie: String,
    #[serde(skip)]
    reference: i32,
}
impl Session {
    pub fn json(self) -> Json<Session> {
        Json(self)
    }

    ///
    /// Pull et creation d'une session a partir email/password/2fa token
    /// 
    pub fn connect(
        password: String,
        email: String,
        code: String,
        conn: &mut DbConnection,
    ) -> Result<Session, Message> {
        match users::table
            .inner_join(level::table)
            .select((users::id, users::password, users::secret, level::value))
            .filter(users::email.eq(email))
            .get_result::<(i32, Option<String>, Option<String>, String)>(conn)
            .optional()
        {
            Ok(Some((id, Some(pass), Some(secret), level))) => {
                if argon2::verify_encoded(pass.as_str(), password.as_bytes()).unwrap_or(false)
                    && GoogleAuthenticator::new().verify_code(secret.as_str(), code.as_str(), 1, 0)
                {
                    diesel::delete(session::table)
                        .filter(session::id_user.eq(id))
                        .execute(conn)
                        .expect("removed");
                    match diesel::insert_into(session::table)
                        .values(session::id_user.eq(id))
                        .returning((session::uuid, session::path))
                        .get_result::<(String, String)>(conn)
                    {
                        Ok((uuid, path)) => Ok(Session {
                            authority: LEVEL::from(level),
                            cookie: uuid,
                            path: path,
                            reference: id,
                        }),
                        _ => Err(Message::unknow()),
                    }
                } else {
                    Err(Message::password_not_math())
                }
            }
            Ok(Some((id, Some(pass), None, level))) => {
                if argon2::verify_encoded(pass.as_str(), password.as_bytes()).unwrap_or(false) {
                    diesel::delete(session::table)
                        .filter(session::id_user.eq(id))
                        .execute(conn)
                        .expect("removed");
                    match diesel::insert_into(session::table)
                        .values(session::id_user.eq(id))
                        .returning((session::uuid, session::path))
                        .get_result::<(String, String)>(conn)
                    {
                        Ok((uuid, path)) => Ok(Session {
                            authority: LEVEL::from(level),
                            cookie: uuid,
                            path: path,
                            reference: id,
                        }),
                        _ => Err(Message::unknow()),
                    }
                } else {
                    Err(Message::password_not_math())
                }
            }
            _ => Err(Message::user_not_match()),
        }
    }

    ///
    /// Pull d'une session a partir d'un cookie et d'un path
    /// 
    pub fn get_session(
        cookie: String,
        path: String,
        conn: &mut DbConnection,
    ) -> Result<Session, Message> {
        match users::table
            .inner_join(level::table)
            .inner_join(session::table)
            .select((users::id, level::value))
            .filter(session::uuid.eq(cookie.to_string()))
            .filter(session::path.eq(path.to_string()))
            .get_result::<(i32, String)>(conn)
            .optional()
        {
            Ok(Some((id, level))) => Ok(Session {
                authority: LEVEL::from(level),
                cookie,
                path,
                reference: id,
            }),
            Ok(None) => Err(Message::session_not_found()),
            _ => Err(Message::unknow()),
        }
    }

    ///
    /// Remove a session by cookie and path
    /// 
    pub fn remove_session(
        cookie: String,
        path: String,
        conn: &mut DbConnection,
    ) -> Result<(), Message> {
        diesel::delete(session::table)
            .filter(session::path.eq(path).and(session::uuid.eq(cookie)))
            .execute(conn)
            .map_or_else(
                |_| Err(Message::unknow()),
                |nbr| {
                    if nbr == 1 {
                        Ok(())
                    } else {
                        Err(Message::session_not_found())
                    }
                },
            )
    }

    ///
    /// Pull d'une session a partir d'un cookie uniquement; !!! attention a la sécurité !!!
    /// 
    pub fn get_session_with_nosecure(
        cookie: String,
        conn: &mut DbConnection,
    ) -> Result<Session, Message> {
        match users::table
            .inner_join(level::table)
            .inner_join(session::table)
            .select((users::id, level::value, session::path))
            .filter(session::uuid.eq(cookie.to_string()))
            .get_result::<(i32, String, String)>(conn)
            .optional()
        {
            Ok(Some((reference, level, path))) => Ok(Session {
                authority: LEVEL::from(level),
                cookie,
                path,
                reference,
            }),
            Ok(None) => Err(Message::session_not_found()),
            _ => Err(Message::unknow()),
        }
    }

    ///
    /// Pull d'une session a partir d'un cookie et d'un path ainsi qu'un niveau d'utilisateur
    /// 
    pub fn get_session_by_level(
        cookie: String,
        path: String,
        level: LEVEL,
        conn: &mut DbConnection,
    ) -> Result<Session, Message> {
        match users::table
            .inner_join(level::table)
            .inner_join(session::table)
            .select(users::id)
            .filter(session::uuid.eq(cookie.to_string()))
            .filter(session::path.eq(path.to_string()))
            .filter(level::value.eq(level.to_string()))
            .get_result::<i32>(conn)
            .optional()
        {
            Ok(Some(reference)) => Ok(Session {
                authority: level,
                cookie,
                path,
                reference,
            }),
            Ok(None) => Err(Message::session_not_found()),
            _ => Err(Message::unknow()),
        }
    }

    ///
    /// get la reference de l'utilisateur (l'id dans la base de donnée)
    /// 
    pub fn get_reference(&self) -> i32 {
        self.reference
    }

    ///
    /// update le mot de passe de l'utilisateur a partir de l'ancien password et le nouveau
    /// 
    pub fn update_password(
        self,
        new_password: String,
        password: String,
        conn: &mut DbConnection,
    ) -> Result<Self, Message> {
        match diesel::update(users::table)
            .set(users::password.eq(new_password))
            .filter(users::id.eq(self.get_reference()))
            .filter(users::password.eq(password))
            .execute(conn)
        {
            Ok(1) => Ok(self),
            _ => Err(Message::unknow()),
        }
    }

    ///
    /// met a jour le temps de la session sans recuperation de la session a partir du path et du cookie
    /// 
    pub fn just_update_time(
        cookie: String,
        path: String,
        conn: &mut DbConnection,
    ) -> Result<(), Message> {
        diesel::update(session::table)
            .set(session::timestamp.eq(now)) //Don't like this
            .filter(session::uuid.eq(cookie.to_string()))
            .filter(session::path.eq(path.to_string()))
            .execute(conn)
            .map_err(|_| Message::unknow())
            .map(|_| ())
    }

    ///
    /// met a jour le mot de passe sans recuperation de la session a partir du path et du cookie, l'ancien mot de passe et le nouveau
    /// 
    pub fn just_update_password(
        new_password: String,
        password: String,
        cookie: String,
        path: String,
        conn: &mut DbConnection,
    ) -> Result<(), Message> {
        Session::get_session(cookie, path, conn).map_or_else(
            |err| Err(err),
            |ok| ok.update_password(new_password, password, conn).map(|_| ()),
        )
    }

    ///
    /// get cookie session
    /// 
    pub fn get_cookie(&self) -> String {
        self.cookie.to_string()
    }

    ///
    /// just said "no cookie found" yeeeeh
    /// 
    pub fn no_cookie() -> Message {
        Message::no_cookie()
    }
}


///
/// Niveau d'elevation utilisateur
/// 
#[derive(PartialEq)]
pub enum LEVEL {
    ADMIN,
    USER,
    NONE,
}
impl LEVEL {
    ///
    /// convert string to level (default none)
    /// 
    fn from(value: String) -> LEVEL {
        match value.as_str() {
            "ADMIN" => LEVEL::ADMIN,
            "USER" => LEVEL::USER,
            _ => LEVEL::NONE,
        }
    }
    ///
    /// level to string
    /// 
    fn to_string(&self) -> String {
        match self {
            LEVEL::ADMIN => "ADMIN",
            LEVEL::USER => "USER",
            LEVEL::NONE => "NONE",
        }
        .to_string()
    }
}


impl Serialize for LEVEL {
    ///
    /// to serialize a level type
    /// 
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            s => serializer.serialize_str(s.to_string().as_str()),
        }
    }
}