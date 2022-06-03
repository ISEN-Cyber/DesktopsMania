#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::dsl::{now, exists};
use diesel::pg::PgConnection;
use diesel::{prelude::*, select};
use core::time;
use std::env;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

table! {
    vemmion (uuid) {
        uuid -> Text,
        timestamp -> Timestamp,
        id_user -> Integer,
        id_vm -> Integer,
        runing -> Bool,
    }
}
table! {
    vm (id) {
        id -> Integer,
        id_creator -> Integer,
        title -> Text,
        timestamp -> Nullable<Timestamp>,
        link -> Text,
        ip -> Nullable<Text>,
    }
}
joinable!(vemmion -> vm (id_vm));
allow_tables_to_appear_in_same_query!(vm, vemmion);

///
/// link read and write as asyn
/// 
fn map_bis(mut reader: TcpStream, mut writer: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let mut buffer = [0; 10];
        match reader.read(&mut buffer[..]) {
            Ok(0) => {
                writer.shutdown(Shutdown::Both).unwrap();
                break;
            }
            Ok(towrite) => match writer.write(&buffer[..towrite]) {
                Ok(0) => panic!(),
                Ok(_) => (),
                Err(_) => panic!(),
            },
            Err(_) => panic!(),
        };
    })
}

///
/// all 20 second verify the bdd to know if you do remove of not the connection
/// 
fn map_stop(stream: TcpStream, uuid: String) -> JoinHandle<()> {
    thread::spawn(move || loop {
        if !select(exists(vemmion::table.filter(vemmion::uuid.eq(uuid.to_string()))))
            .get_result(&establish_connection())
            .ok().unwrap_or(false)
        {
            stream.shutdown(Shutdown::Both).unwrap();
            break;
        }
        thread::sleep(time::Duration::from_secs(20))
    })
}


///
/// Redirect read and write of stream the correct vnc server
/// 
fn handle_client(mut stream: TcpStream, connection: &PgConnection) {
    let mut starter = [0; 100];

    match match stream.read(&mut starter[..]) {
        Ok(towrite) => diesel::update(vemmion::table)
            .filter(
                vemmion::uuid.eq(String::from_utf8(starter[..towrite].to_vec())
                    .expect("this is not a string")
                    .trim()),
            )
            .filter(
                vemmion::timestamp.gt(diesel::select(now).get_result::<SystemTime>(&establish_connection()).unwrap()
                    .checked_sub(Duration::from_secs(5))
                    .unwrap()),
            )
            .set(vemmion::runing.eq(true))
            .returning((vemmion::id_vm, vemmion::uuid))
            .get_result::<(i32, String)>(connection)
            .map(|(id, uuid)| {
                (
                    vm::table
                        .select(vm::ip)
                        .filter(vm::id.eq(id))
                        .get_result::<Option<String>>(connection)
                        .unwrap(),
                    uuid,
                )
            })
            .map_err(|_| String::from_utf8(starter[..towrite].to_vec()).unwrap()),

        _ => Err("not chaine".to_string()),
    } {
        Ok((Some(addr), uuid)) => match TcpStream::connect(addr) {
            Ok(other) => match (other.try_clone(), stream.try_clone(),stream.try_clone()) {
                (Ok(other2), Ok(stream2),Ok(enocre)) => {
                    map_bis(other, stream);
                    map_stop(enocre,uuid.to_string());
                    match map_bis(stream2, other2).join() {
                        _ => diesel::delete(vemmion::table)
                            .filter(vemmion::uuid.eq(uuid))
                            .execute(connection)
                            .expect("msg"),
                    };
                }
                _ => (),
            },
            _ => (),
        },
        Err(str) => println!("{}", str.trim()),
        _ => println!("We don't have an ip.. strange ?"),
    }
}

///
/// databse connection
/// 
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

///
/// Listen the 5901 port to wait to redirect
/// 
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:5901")?;
    let connection = establish_connection();

    for stream_e in listener.incoming() {
        match stream_e {
            Ok(stream) => handle_client(stream, &connection),
            _ => (),
        };
    }
    Ok(())
}
