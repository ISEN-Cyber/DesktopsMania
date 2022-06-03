use self::diesel::*;
use super::super::diesel;
use super::super::message::Message;
use super::super::schema::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::PgConnection as DbConnection;
use rocket::serde::json::Json;
pub type AfterModif<T = Json<Message>, F = Json<Message>> = Result<T, F>;

///
/// remove the user identifie by the email and the administrator (reference)
/// 
pub fn remove_user(email: String, reference: i32, conn: &mut DbConnection) -> AfterModif {
    match diesel::delete(users::table)
        .filter(users::email.eq(email))
        .filter(users::id_admin.eq(reference))
        .execute(conn)
    {
        Ok(1) => Ok(Message::ok_user_remove().json()),
        Ok(0) => Err(Message::bad_remove_user_never_existe().json()),
        _ => Err(Message::unknow().set_extra("dfn".to_string()).json()),
    }
}

///
/// add a user with basic info and administrator reference
/// 
pub fn add_user(
    email: String,
    firstname: String,
    lastname: String,
    reference: i32,
    conn: &mut DbConnection,
) -> AfterModif {
    match diesel::insert_into(users::table)
        .values((
            users::email.eq(email.to_string()),
            users::first_name.eq(firstname.to_string()),
            users::last_name.eq(lastname.to_string()),
            users::id_admin.eq(reference),
            users::id_level.eq(0),
        ))
        .execute(conn)
    {
        Ok(1) => Ok(Message::ok_user_added().json()),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err(Message::bad_user_yet_existe().json())
        }
        _ => Err(Message::unknow().json()),
    }
}

///
/// Reset all user session and password - it's like a new account
/// 
pub fn mod_password(email:String,reference: i32, conn: &mut DbConnection) -> AfterModif {
    match diesel::update(users::table).filter(users::id_admin.eq(reference)).filter(users::email.eq(email)).set(users::password.eq::<Option<String>>(None)).returning(users::id).get_result::<i32>(conn)
    {
        Ok(id) => { 
            diesel::delete(session::table).filter(session::id_user.eq(id)).execute(conn).ok();
            diesel::delete(vemmion::table).filter(vemmion::id_user.eq(id)).execute(conn).ok();
            diesel::delete(temporaire::table).filter(temporaire::id_user.eq(id)).execute(conn).ok();
            Ok(Message::ok_user_added().json())
        },
        _ => Err(Message::unknow().json())
    }
}
