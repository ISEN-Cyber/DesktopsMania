#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
use diesel::dsl::{now, not, exists};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as MessageEmail, SmtpTransport, Transport};
use random_string::generate;
use serde::Deserialize;
use std::env;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
mod schema;
use argon2;
use schema::*;


//
// Set all env variable as static
//
lazy_static! {
    static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    static ref EMAIL_PASS: String = env::var("EMAIL_PASS").expect("EMAIL_PASS must be set");
    static ref EMAIL: String = env::var("EMAIL").expect("EMAIL must be set");
    static ref ROOT_CERT: String = env::var("ROOT_CERT").expect("ROOT_CERT must be set");
    static ref SMTP_SERV: String = env::var("SMTP_SERV").unwrap_or(String::from("smtp.gmail.com"));
    static ref SERVER_URL: String =
        env::var("SERVER_URL").unwrap_or(String::from("[serveur adress]"));
    static ref USERS: Vec<Users> =
        serde_json::from_str(env::var("USERS").unwrap_or(String::from("[]")).as_str())
            .unwrap_or(vec![]);
    static ref TIME_SESSION: Duration = Duration::from_secs(
        env::var("TIME_SESSION")
            .unwrap_or_default()
            .parse()
            .unwrap_or(60 * 5)
    );
    static ref TIME_TEMPORAIRE: Duration = Duration::from_secs(
        env::var("TIME_TEMPORAIRE")
            .unwrap_or_default()
            .parse()
            .unwrap_or(60 * 60 * 24)
    );
    static ref TIME_VEMMION: Duration = Duration::from_secs(
        env::var("TIME_VEMMION")
            .unwrap_or_default()
            .parse()
            .unwrap_or(5)
    );
    static ref TIME_VEMMION_MAX: Duration = Duration::from_secs(
        env::var("TIME_VEMMION_MAX")
            .unwrap_or_default()
            .parse()
            .unwrap_or(60 * 60 * 24)
    );
    static ref CHANGE_PASSWORD: Duration = Duration::from_secs(
        env::var("CHANGE_PASSWORD")
            .unwrap_or_default()
            .parse()
            .unwrap_or(60 * 60 * 24 * 360)
    );
    static ref ACTUALISATION: Duration = Duration::from_secs(
        env::var("ACTUALISATION")
            .unwrap_or_default()
            .parse()
            .unwrap_or(60 * 5)
    );
}

#[derive(QueryableByName, Queryable, Insertable, Clone, Deserialize)]
#[table_name = "users"]
struct Users {
    email: String,
    first_name: String,
    last_name: String,
    password: Option<String>,
    id_level: i32,
}
impl Users {
    ///
    /// hash the password user
    /// 
    fn set_password(mut self) -> Self {
        self.password = Some(
            argon2::hash_encoded(
                self.password.unwrap().as_bytes(),
                generate(
                    16,
                    "azertyuiopqsdfghjklmwxcvbn123456789AZERTYUIOPQSDFGHJKLMWXCVBN?,;.:/!§$£¤^¨ù%*µ~#'{([-|è`_\\ç^à@)]=}",
                )
                .as_bytes(),
                &argon2::Config::default(),
            )
            .expect(""),
        );
        self
    }
}

///
/// do the sql migration
/// 
fn run_migrations(connection: &PgConnection) -> Result<(), diesel_migrations::RunMigrationsError> {
    embed_migrations!();
    embedded_migrations::run(connection)
}

///
/// database connection
/// 
fn establish_connection() -> PgConnection {
    PgConnection::establish(&DATABASE_URL).expect(&format!("Error connecting to {}", *DATABASE_URL))
}

///
/// flush default account (and ignore if existe)
/// 
fn run_default_admin(
    connection: &PgConnection,
    mut users: Vec<Users>,
) -> Result<Vec<String>, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(
            users
                .iter_mut()
                .map(|usered| usered.clone().set_password())
                .collect::<Vec<Users>>(),
        )
        .on_conflict_do_nothing()
        .returning(users::email)
        .get_results::<String>(connection)
}

///
/// manage the bdd recurently (remove session, set temporary session, send email...)
/// 
fn main() {
    run_migrations(&establish_connection()).expect("oh non");
    let manage = run_default_admin(&establish_connection(), USERS.to_vec()).expect("bad things");
    manage
        .iter()
        .for_each(|email| println!("the user with the email {} was create", email));
    loop {
        sleep(*ACTUALISATION);
        let nowed = diesel::select(now).get_result::<SystemTime>(&establish_connection()).unwrap();
        println!("{:?}",nowed);
        diesel::delete(session::table)
            .filter(session::timestamp.lt(nowed.checked_sub(*TIME_SESSION).unwrap()))
            .execute(&establish_connection())
            .expect("session");
        diesel::delete(vemmion::table)
            .filter(
                (vemmion::timestamp
                    .lt(nowed.checked_sub(*TIME_VEMMION).unwrap())
                    .and(vemmion::runing.eq(false)))
                .or(vemmion::timestamp
                    .lt(nowed.checked_sub(*TIME_VEMMION_MAX).unwrap())),
            )
            .execute(&establish_connection())
            .expect("vemmion");
        diesel::delete(temporaire::table)
            .filter(
                temporaire::timestamp.lt(nowed.checked_sub(*TIME_TEMPORAIRE).unwrap()),
            )
            .execute(&establish_connection())
            .expect("temporaire");
        diesel::update(users::table)
            .filter(users::timestamp.lt(nowed.checked_sub(*CHANGE_PASSWORD).unwrap()))
            .set((
                users::password.eq::<Option<String>>(None),
                users::timestamp.eq(nowed),
            ))
            .execute(&establish_connection())
            .expect("change password");

        match users::table
            .select((
                users::id,
                users::email,
                users::first_name,
                users::last_name,
                users::id_admin,
            ))
            .filter(users::password.is_null())
            .filter(not(exists(temporaire::table.filter(temporaire::id_user.eq(users::id)))))
            .get_results::<(i32, String, String, String, Option<i32>)>(&establish_connection())
        {
            Ok(vector) => 
            
            vector
                .iter()
                .for_each(|(id, email, firstname, lastname, id_admin)| {
                    send_email(
                        email.to_string(),
                        firstname.to_string(),
                        lastname.to_string(),
                        users::table
                            .select(users::email)
                            .filter(users::id.eq(id_admin.unwrap_or(*id)))
                            .get_result::<String>(&establish_connection())
                            .expect("un email"),
                        diesel::insert_into(temporaire::table)
                            .values(temporaire::id_user.eq(id))
                            .returning(temporaire::path)
                            .get_result(&establish_connection())
                            .expect("very strange"),
                    )
                } 
            ),
            _ => todo!(),
        };
    }
}

///
/// Send an email with certificat by a smtp serveur 
/// 
fn send_email(
    email: String,
    firstname: String,
    lastname: String,
    creator_email: String,
    path: String,
) {
    let filename = String::from("certificate.crt");
    let filebody = ROOT_CERT.to_string();
    let content_type = ContentType::parse("application/pdf").unwrap();
    let emailed = MessageEmail::builder()
        .from(
            format!("desktop.mania.isen <{}>", *EMAIL)
                .parse()
                .unwrap(),
        )
        .reply_to(format!("desktop.mania.isen <{}>", *EMAIL).parse().unwrap())
        .to(format!("{} {}<{}>",firstname,lastname,email).parse().unwrap())
        .subject("desktopMania new account")
        .multipart(MultiPart::mixed()
        .singlepart(SinglePart::plain(format!("Your path to cpy past in the navigator is {}/{}/{} --- If you want more information, ask your administrator {}",*SERVER_URL,"extra/temporaire",path,creator_email)))
        .singlepart(Attachment::new(filename).body(filebody, content_type)))
        .unwrap();

    let creds = Credentials::new(EMAIL.to_string(), EMAIL_PASS.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&SMTP_SERV)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&emailed) {
        Ok(_) => println!("The email for {} was send", email),
        Err(e) => println!(
            "The email for {} was not send. His path credential is {}, raison:{:?}",
            email, path,e
        ),
    }
}
