use std::time::SystemTime;
use super::super::message::Message;
use self::diesel::*;
use super::super::diesel;
use super::super::schema::*;
use diesel::PgConnection as DbConnection;
use rocket::serde::Serialize;




#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    email: String,
    first_name: String,
    last_name: String,
    have_password: bool,
    have_secret: bool,
    level: String,
    lasttime: SystemTime,
    code: Option<String>,
    timetemp: Option<SystemTime>,
    timesession: Option<SystemTime>,
}
impl User {
    ///
    /// set user information
    /// 
    pub fn set(
        email: String,
        first_name: String,
        last_name: String,
        have_password: bool,
        have_secret: bool,
        level: String,
        lasttime: SystemTime,
        code: Option<String>,
        timetemp: Option<SystemTime>,
        timesession: Option<SystemTime>,
    ) -> User {
        User {
            email,
            first_name,
            last_name,
            have_password,
            have_secret,
            level,
            lasttime,
            code,
            timetemp,
            timesession,
        }
    }
}

///
/// recupere les informations de tout les utilisateurs lié a un administrateur
/// 
pub fn all_user(reference: i32, conn: &mut DbConnection) -> Result<Vec<User>,Message> {
    match users::table
        .inner_join(level::table)
        .left_outer_join(temporaire::table)
        .left_outer_join(session::table)
        //.left_outer_join(link_user_vm::table).group_by(users::id)
        .select((
            users::email,
            users::first_name,
            users::last_name,
            users::password,
            users::secret,
            users::timestamp,
            level::value,
            temporaire::code.nullable(),
            temporaire::timestamp.nullable(),
            session::timestamp.nullable(),
            //count(link_user_vm::id_vm).nullable()
        ))
        .filter(users::id_admin.eq(reference))
        .get_results::<(
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            SystemTime,
            String,
            Option<String>,
            Option<SystemTime>,
            Option<SystemTime>,
        )>(conn)
    {
        Ok(data) => Ok(data
            .iter()
            .map(
                |(
                    email,
                    firstname,
                    lastname,
                    password,
                    secret,
                    lasttime,
                    level,
                    code,
                    timetemp,
                    timesession,
                )| {
                    User::set(
                        email.to_string(),
                        firstname.to_string(),
                        lastname.to_string(),
                        password.is_some(),
                        secret.is_some(),
                        level.to_string(),
                        *lasttime,
                        code.as_ref().map(|st| st.to_string()),
                        *timetemp,
                        *timesession,
                    )
                },
            )
            .collect()),
        Err(_) => Err(Message::unknow()),
    }
}

///
/// recupere les inforations d'un utilisateur
/// 
pub fn me(reference: i32, conn: &mut DbConnection) -> Result<User,Message> {
    match users::table
        .inner_join(level::table)
        .left_outer_join(temporaire::table)
        .left_outer_join(session::table)
        .filter(users::id.eq(reference))
        .select((
            users::email,
            users::first_name,
            users::last_name,
            users::password,
            users::secret,
            users::timestamp,
            level::value,
            temporaire::code.nullable(),
            temporaire::timestamp.nullable(),
            session::timestamp.nullable(),
            //count(link_user_vm::id_vm).nullable()
        ))
        .get_result::<(
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            SystemTime,
            String,
            Option<String>,
            Option<SystemTime>,
            Option<SystemTime>,
        )>(conn)
    {
        Ok((
            email,
            firstname,
            lastname,
            password,
            secret,
            lasttime,
            level,
            code,
            timetemp,
            timesession,
        )) => Ok(User::set(
            email.to_string(),
            firstname.to_string(),
            lastname.to_string(),
            password.is_some(),
            secret.is_some(),
            level.to_string(),
            lasttime,
            code.as_ref().map(|st| st.to_string()),
            timetemp,
            timesession,
        )),
        _ => Err(Message::unknow()),
    }
}

///
/// recupere les informations d'un utilisateur a partir de l'email et de l'id d'un administrateur
/// 
pub fn you(email:String,reference: i32, conn: &mut DbConnection) -> Result<User,Message> {
    match users::table
        .inner_join(level::table)
        .left_outer_join(temporaire::table)
        .left_outer_join(session::table)
        .filter(users::id_admin.eq(reference))
        .filter(users::email.eq(email))
        .select((
            users::email,
            users::first_name,
            users::last_name,
            users::password,
            users::secret,
            users::timestamp,
            level::value,
            temporaire::code.nullable(),
            temporaire::timestamp.nullable(),
            session::timestamp.nullable(),
        ))
        .get_result::<(
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            SystemTime,
            String,
            Option<String>,
            Option<SystemTime>,
            Option<SystemTime>,
        )>(conn)
    {
        Ok((
            email,
            firstname,
            lastname,
            password,
            secret,
            lasttime,
            level,
            code,
            timetemp,
            timesession,
        )) => Ok(User::set(
            email.to_string(),
            firstname.to_string(),
            lastname.to_string(),
            password.is_some(),
            secret.is_some(),
            level.to_string(),
            lasttime,
            code.as_ref().map(|st| st.to_string()),
            timetemp,
            timesession,
        )),
        _ => Err(Message::unknow()),
    }
}