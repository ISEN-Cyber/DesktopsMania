mod extra;
mod message;
mod schema;
mod session;
mod user_gestion;
mod virtual_machine;

use rocket::config::{Config, TlsConfig, Ident, SecretKey, Shutdown, CipherSuite};

use rocket::data::Limits;
use rocket::fairing::AdHoc;

use rocket::figment::providers::Env;
use rocket::figment::{Profile, Figment};
use rocket::{fs::NamedFile, request::FlashMessage};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::env;
use std::net::{Ipv4Addr, IpAddr};
use rocket::http::Status;
use rocket::Request;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;


#[catch(default)]
fn general_catch(_status:Status, _request:&Request) -> String
{
    "Rater".to_string()
}


#[get("/")]
fn index(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render(
        "index",
        HashMap::<String, String>::from(
            flash.map_or([("".to_string(), "".to_string())], |message| {
                [(message.kind().to_string(), message.message().to_string())]
            }),
        ),
    )
}

#[get("/base.css")]
async fn css() -> Option<NamedFile> {
    NamedFile::open("static/index.css").await.ok()
}

#[get("/base.js")]
async fn js() -> Option<NamedFile> {
    NamedFile::open("static/index.js").await.ok()
}

#[get("/complete.css")]
async fn css_1() -> Option<NamedFile> {
    NamedFile::open("static/features.css").await.ok()
}

#[get("/complete.js")]
async fn js_1() -> Option<NamedFile> {
    NamedFile::open("static/features.js").await.ok()
}

#[get("/favicon.ico")]
async fn ico() -> Option<NamedFile> {
    NamedFile::open("static/image.ico").await.ok()
}

#[get("/user.png")]
async fn user_png() -> Option<NamedFile> {
    NamedFile::open("static/image/user.png").await.ok()
}

#[get("/vm.png")]
async fn vm_png() -> Option<NamedFile> {
    NamedFile::open("static/image/vm.png").await.ok()
}

#[launch]
fn rocket() -> _ {
    let tls_config = TlsConfig::from_paths(
        "private/rsa_sha256_cert.pem",
        "private/rsa_sha256_key.pem",
    ).with_preferred_server_cipher_order(true)
    .with_ciphers(
        CipherSuite::TLS_V13_SET
    );

    let config = Config {
        tls: Some(tls_config),
        profile: Profile::Default,
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port: 8000,
        workers: num_cpus::get(),
        keep_alive: 5,
        limits: Limits::default(),
        ident: Ident::try_new("Desktop Mania").unwrap(),
        secret_key: SecretKey::derive_from(env::var("SECRET_KEY").expect("please").as_bytes()),
        temp_dir: std::env::temp_dir().into(),
        log_level: rocket::log::LogLevel::Off,
        shutdown: Shutdown::default(),
        cli_colors: false,
        __non_exhaustive: (),
    };

     let new=Figment::from(config)
            .merge(Env::prefixed("ROCKET_").ignore(&["PROFILE"]).global());
    
    rocket::custom(new)
        .mount(
            "/",
            routes![index, css, js, ico, css_1, js_1, vm_png, user_png],
        )
        .register(
            "/",
            catchers![general_catch]
        )
        .attach(schema::DataBase::fairing())
        .attach(user_gestion::stage())
        .attach(virtual_machine::stage())
        .attach(extra::stage())
        .attach(Template::fairing())
}
