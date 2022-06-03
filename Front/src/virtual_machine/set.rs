use self::diesel::*;
use super::super::diesel;
use super::super::message::Message;
use super::super::message::MessageJson;
use super::super::schema::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::PgConnection as DbConnection;


///
/// ajoute une nouveau machine virtuelle
/// 
pub fn add_vm(name: String, reference: i32, link: String, conn: &mut DbConnection) -> MessageJson {
    match diesel::insert_into(vm::table)
        .values((
            vm::id_creator.eq(reference),
            vm::link.eq(link),
            vm::title.eq(name),
        ))
        .execute(conn)
    {
        Ok(1) => Message::unknow().json(),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Message::unknow().json(),
        _ => Message::unknow().json(),
    }
}

///
/// supprimer une machine virtuelle 
/// 
pub fn remove_vm(
    name: String,
    reference: i32,
    conn: &mut DbConnection,
) -> Result<String, diesel::result::Error> {
    diesel::delete(vm::table)
        .filter(vm::id_creator.eq(reference))
        .filter(vm::title.eq(name))
        .returning(vm::link)
        .get_result::<String>(conn)
}

///
/// recupere le lien a kubevirt d'une vm
/// 
pub fn get_vm(
    name: String,
    reference: i32,
    conn: &mut DbConnection,
) -> Result<String, diesel::result::Error> {
    vm::table
        .select(vm::link)
        .filter(vm::id_creator.eq(reference))
        .filter(vm::title.eq(name))
        .get_result::<String>(conn)
}

///
/// met a jour l'ip d'une vm
/// 
pub fn ip(name: String, ip: String, reference: i32, conn: &mut DbConnection) -> MessageJson {
    diesel::update(vm::table)
        .filter(vm::id_creator.eq(reference))
        .filter(vm::title.eq(name))
        .set(vm::ip.eq(ip))
        .execute(conn)
        .ok();
    Message::unknow().json()
}

///
/// remove l'ip d'une vm (state off)
/// 
pub fn ip_stop(name: String, reference: i32, conn: &mut DbConnection) -> MessageJson {
    let id = diesel::update(vm::table)
        .filter(vm::id_creator.eq(reference))
        .filter(vm::title.eq(name))
        .set(vm::ip.eq::<Option<String>>(None))
        .returning(vm::id)
        .get_result::<i32>(conn)
        .ok();
    if id.is_some() {
        diesel::delete(vemmion::table)
            .filter(vemmion::id_vm.eq(id.unwrap()))
            .execute(conn)
            .ok();
    }
    Message::unknow().json()
}

///
/// Associe un utilisateur a une machine
/// 
pub fn associate(
    name: String,
    email: String,
    reference: i32,
    conn: &mut DbConnection,
) -> MessageJson {
    let id_user = users::table
        .select(users::id)
        .filter(users::id_admin.eq(reference))
        .filter(users::email.eq(email))
        .get_result::<i32>(conn);

    let id_vm = vm::table
        .select(vm::id)
        .filter(vm::id_creator.eq(reference))
        .filter(vm::title.eq(name))
        .get_result::<i32>(conn);
    match (id_user, id_vm) {
        (Ok(idu), Ok(idv)) => diesel::insert_into(link_user_vm::table)
            .values((link_user_vm::id_user.eq(idu), link_user_vm::id_vm.eq(idv)))
            .execute(conn)
            .map_or_else(|_| Message::unknow().json(), |_| Message::unknow().json()),
        _ => Message::unknow().json(),
    }
}

///
/// supprimer la relation entre un utilisateur et une machine
/// 
pub fn remove_associate(
    name: String,
    email: String,
    reference: i32,
    conn: &mut DbConnection,
) -> MessageJson {
    let id_user = users::table
        .select(users::id)
        .filter(users::id_admin.eq(reference))
        .filter(users::email.eq(email))
        .get_result::<i32>(conn);

    let id_vm = vm::table
        .select(vm::id)
        .filter(vm::id_creator.eq(reference))
        .filter(vm::title.eq(name))
        .get_result::<i32>(conn);
    match (id_user, id_vm) {
        (Ok(idu), Ok(idv)) => diesel::delete(link_user_vm::table)
            .filter(link_user_vm::id_user.eq(idu))
            .filter(link_user_vm::id_vm.eq(idv))
            .execute(conn)
            .map_or_else(|_| Message::unknow().json(), |_| Message::unknow().json()),
        _ => Message::unknow().json(),
    }
}
