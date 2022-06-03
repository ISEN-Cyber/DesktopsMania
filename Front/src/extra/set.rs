use self::diesel::*;
use super::super::diesel;
use super::super::schema::*;
use diesel::PgConnection as DbConnection;

///
/// cree le nouveau mot de passe d'un utilisateur avec son secret et le token de l'administrateur
/// 
pub fn password(
    path: String,
    password: String,
    code: String,
    secret: String,
    conn: &mut DbConnection,
) -> Result<(), ()> {
    match temporaire::table
        .select(temporaire::id_user)
        .filter(temporaire::path.eq(path))
        .filter(temporaire::code.eq(code))
        .get_result::<i32>(conn)
        .optional()
    {
        Ok(Some(id)) => {
            match diesel::update(users::table)
                .set((users::password.eq(password),users::secret.eq(secret)))
                .filter(users::id.eq(id))
                .execute(conn)
            {
                Ok(1) => {
                    diesel::delete(temporaire::table).filter(temporaire::id_user.eq(id)).execute(conn).expect("oh non");
                    Ok(())
                },
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}
