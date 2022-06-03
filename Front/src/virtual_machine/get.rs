use self::diesel::*;
use super::super::diesel;
use super::super::message::Message;
use super::super::schema::*;
use diesel::dsl::count;
use diesel::sql_types::*;
use diesel::PgConnection as DbConnection;
use rocket::serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, QueryableByName, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct VirtualMachine {
    #[sql_type = "Text"]
    creator: String,
    #[sql_type = "Text"]
    title: String,
    #[sql_type = "Nullable<Timestamp>"]
    timestamp: Option<SystemTime>,
    #[sql_type = "Bool"]
    state: bool,
}

///
/// communication avec kubevirt afin de crée une machine
/// 
pub async fn create_machine(_os: String, _name: String, _id: i32) -> Result<String, reqwest::Error> {
    Ok("not a link".to_string())
    //Ok(reqwest::get("").await?.text().await?)
}

///
/// communication avec kubevirt afin d'allumer une machine
/// 
pub async fn start_machine(_link: String) -> Result<String, reqwest::Error> {
    Ok("vnc:5901".to_string())
    //Ok(reqwest::get("").await?.text().await?)
}

///
/// communication avec kubevirt afin de stopper une machine
/// 
pub async fn stop_machine(_link: String) -> Result<String, reqwest::Error> {
    Ok("ok".to_string())
    //Ok(reqwest::get("").await?.text().await?)
}

///
/// communication avec kubevirt afin de supprimer une machine
/// 
pub async fn delete_machine(_link: String) -> Result<String, reqwest::Error> {
    Ok("ok".to_string())
    //Ok(reqwest::get("").await?.text().await?)
}

///
/// creation et recuperation du path pour l'utilisation du vnc
/// 
pub fn vemmion(name: String, reference: i32, conn: &mut DbConnection) -> Result<String, Message> {
    diesel::insert_into(vemmion::table)
        .values((
            vemmion::id_user.eq(reference),
            vemmion::id_vm.eq(vm::table
                .select(vm::id)
                .filter(vm::title.eq(name))
                .get_result::<i32>(conn)
                .expect("msg")),
        ))
        .returning(vemmion::uuid)
        .get_result::<String>(conn)
        .map_err(|_| Message::unknow())
}

type SomeMachine = Result<Vec<VirtualMachine>, Message>;

///
/// recupere les information des machines appartenant a l'utilisateur
/// 
pub fn me(reference: i32, conn: &mut DbConnection) -> SomeMachine {
    vm::table
        .inner_join(link_user_vm::table)
        .inner_join(users::table)
        .filter(link_user_vm::id_user.eq(reference))
        .select((users::email, vm::title, vm::timestamp,vm::ip.is_not_null()))
        .get_results::<VirtualMachine>(conn)
        .map_err(|_| Message::unknow())
}

///
/// recupere les information des machines appartenant au createur de la machine
/// 
pub fn you(reference: i32, conn: &mut DbConnection) -> SomeMachine {
    vm::table
        .inner_join(users::table)
        .filter(vm::id_creator.eq(reference))
        .select((users::email, vm::title, vm::timestamp,vm::ip.is_not_null()))
        .get_results::<VirtualMachine>(conn)
        .map_err(|_| Message::unknow())
}


///
/// compte le nombre d'utilisateur présent sur une machine
/// 
pub fn user_on_vm_juste_chiffre(name: String, conn: &mut DbConnection) -> i64 {
    vm::table
        .inner_join(link_user_vm::table)
        .filter(vm::title.eq(name))
        .select(count(link_user_vm::id_user))
        .get_result(conn)
        .map_or_else(|_| 0,|ok|ok)
}

///
/// List des utilisateurs sur une machine
/// 
pub fn user_on_vm_for_admin(reference: i32,name: String, conn: &mut DbConnection) -> Vec<String> {
    vm::table
        .inner_join(link_user_vm::table.inner_join(users::table))
        .filter(vm::title.eq(name))
        .filter(vm::id_creator.eq(reference))
        .select(users::email)
        .get_results::<String>(conn)
        .map_or_else(|_| vec![],|ok|ok)
}



