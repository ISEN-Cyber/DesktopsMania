use super::message::MessageJson;
use super::schema::DataBase;
use super::session::*;
use super::AdHoc;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
mod formulaire;
mod get;
mod set;
use formulaire::{ConnectForm, CreateUserForm, RemoveUserForm, UpdatePasswordForm};
use rocket::response::status::Forbidden;

type ResultOfAction<T = Result<JsonInfo, MessageJson>, E = Forbidden<MessageJson>> =
    std::result::Result<T, E>;

type ResultOfUpdate<T = Result<(), MessageJson>, E = Forbidden<MessageJson>> =
    std::result::Result<T, E>;

type ResultOfGetUser<T = Json<Vec<get::User>>, E = Forbidden<MessageJson>> =
    std::result::Result<T, E>;

type ResultOfUser<T = Json<get::User>, E = Forbidden<MessageJson>> = std::result::Result<T, E>;

type ResultOfModification<T = set::AfterModif, E = Forbidden<MessageJson>> =
    std::result::Result<T, E>;

#[get("/disconnect/<path>")]
async fn disconnect(path: String, cookies: &CookieJar<'_>, db: DataBase) -> Status {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| Session::remove_session(data, path, conn))
                .await
                .map_or_else(
                    |_| Status::NotFound,
                    |_| {
                        cookies.remove_private(cookie);
                        Status::Accepted
                    },
                )
        }
        None => Status::NotFound,
    }
}

#[get("/actualise/<path>")]
async fn actualise(path: String, cookies: &CookieJar<'_>, db: DataBase) -> Status {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| Session::just_update_time(data, path, conn))
                .await
                .map_or_else(|_| Status::NotFound, |_| Status::Accepted)
        }
        None => Status::NotFound,
    }
}

#[get("/connect")]
async fn connect_auto(cookies: &CookieJar<'_>, db: DataBase) -> Result<JsonInfo, MessageJson> {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| {
                Session::get_session_with_nosecure(data, conn)
                    .map_or_else(|err| Err(err), |ok| Ok(ok))
            })
            .await
            .map_err(|err| {
                cookies.remove_private(cookie);
                err.json()
            })
            .map(|ok| ok.json())
        }
        None => Err(Session::no_cookie().json()),
    }
}

#[get("/all/<path>")]
async fn see_all_user(path: String, cookies: &CookieJar<'_>, db: DataBase) -> ResultOfGetUser {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| {
                match Session::get_session_by_level(data, path, LEVEL::ADMIN, conn) {
                    Ok(session) => get::all_user(session.get_reference(), conn)
                        .map(|ok| Json(ok))
                        .map_err(|err| Forbidden(Some(err.json()))),

                    Err(_) => Err(Forbidden(None)),
                }
            })
            .await
        }
        _ => Err(Forbidden(None)),
    }
}

#[post("/connect", data = "<connect_form>")]
async fn connect(
    connect_form: ConnectForm,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> ResultOfAction {
    match (
        connect_form.email(),
        connect_form.password(),
        connect_form.code(),
    ) {
        (Ok(email), Ok(password), Ok(code)) => {
            match db
                .run(move |conn| {
                    Session::connect(password, email, code, conn)
                        .map_or_else(|err| Err(err), |ok| Ok(ok))
                })
                .await
            {
                Ok(session) => {
                    cookies
                        .get_private("session")
                        .map(|cookie| cookies.remove_private(cookie));
                    cookies.add_private(Cookie::new("session", session.get_cookie()));
                    Ok(Ok(session.json()))
                }
                Err(err_session) => Ok(Err(err_session.json())),
            }
        }
        (Err(error), _, _) => Err(Forbidden(Some(error.json()))),
        (_, Err(error), _) => Err(Forbidden(Some(error.json()))),
        (_, _, Err(error)) => Err(Forbidden(Some(error.json()))),
    }
}

#[post("/update/<path>", data = "<update_password>")]
async fn update(
    update_password: UpdatePasswordForm,
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> ResultOfUpdate {
    match (
        cookies.get_private("session"),
        update_password.new_password(),
        update_password.password(),
    ) {
        (Some(cookie), Ok(new_password), Ok(password)) => {
            let data = cookie.value().to_string();
            Ok(db
                .run(move |conn| {
                    Session::just_update_password(new_password, password, data, path, conn)
                })
                .await
                .map_err(|err| err.json())
                .map(|_| ()))
        }
        (_, Err(value), _) => Err(Forbidden(Some(value.json()))),
        (_, _, _) => Err(Forbidden(None)),
    }
}

#[post("/delete/<path>", data = "<remove_user>")]
async fn del(
    remove_user: RemoveUserForm,
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> ResultOfModification {
    match (cookies.get_private("session"), remove_user.email()) {
        (Some(cookie), Ok(email)) => {
            let data = cookie.value().to_string();

            db.run(move |conn| {
                match Session::get_session_by_level(data, path, LEVEL::ADMIN, conn) {
                    Ok(session) => Ok(set::remove_user(email, session.get_reference(), conn)),
                    Err(_) => Err(Forbidden(None)),
                }
            })
            .await
        }
        (_, Err(value)) => Err(Forbidden(Some(value.json()))),
        (_, _) => Err(Forbidden(None)),
    }
}

#[post("/remove/<path>", data = "<remove_user>")]
async fn remove(
    remove_user: RemoveUserForm,
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> ResultOfModification {
    match (cookies.get_private("session"), remove_user.email()) {
        (Some(cookie), Ok(email)) => {
            let data = cookie.value().to_string();

            db.run(move |conn| {
                match Session::get_session_by_level(data, path, LEVEL::ADMIN, conn) {
                    Ok(session) => Ok(set::mod_password(email, session.get_reference(), conn)),
                    Err(_) => Err(Forbidden(None)),
                }
            })
            .await
        }
        (_, Err(value)) => Err(Forbidden(Some(value.json()))),
        (_, _) => Err(Forbidden(None)),
    }
}

#[post("/add/<path>", data = "<create_user>")]
async fn add(
    create_user: CreateUserForm,
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> ResultOfModification {
    match (
        cookies.get_private("session"),
        create_user.email(),
        create_user.firstname(),
        create_user.lastname(),
    ) {
        (Some(cookie), Ok(email), Ok(firstname), Ok(lastname)) => {
            let data = cookie.value().to_string();

            db.run(move |conn| {
                match Session::get_session_by_level(data, path, LEVEL::ADMIN, conn) {
                    Ok(session) => Ok(set::add_user(
                        email,
                        firstname,
                        lastname,
                        session.get_reference(),
                        conn,
                    )),
                    Err(_) => Err(Forbidden(None)),
                }
            })
            .await
        }
        (_, Err(value), _, _) => Err(Forbidden(Some(value.json()))),
        (_, _, _, _) => Err(Forbidden(None)),
    }
}

#[get("/get_one/<path>/<email>")]
async fn see_one_user(
    path: String,
    email: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> ResultOfUser {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| {
                match Session::get_session_by_level(data, path, LEVEL::ADMIN, conn) {
                    Ok(session) => get::you(email, session.get_reference(), conn)
                        .map(|ok| Json(ok))
                        .map_err(|err| Forbidden(Some(err.json()))),
                    Err(_) => Err(Forbidden(None)),
                }
            })
            .await
        }
        _ => Err(Forbidden(None)),
    }
}

#[get("/get_me/<path>")]
async fn see_me(path: String, cookies: &CookieJar<'_>, db: DataBase) -> ResultOfUser {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| match Session::get_session(data, path, conn) {
                Ok(session) => get::me(session.get_reference(), conn)
                    .map(|ok| Json(ok))
                    .map_err(|err| Forbidden(Some(err.json()))),
                Err(_) => Err(Forbidden(None)),
            })
            .await
        }
        _ => Err(Forbidden(None)),
    }
}

/*#[get("/info/<path>")]
async fn get_info(path: String, cookies: &CookieJar<'_>, db: DataBase) -> ResultOfUser {
    match cookies.get_private("session") {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db.run(move |conn| match Session::get_session(data, path, conn) {
                Ok(session) => get::me(session.get_reference(), conn)
                    .map(|ok| Json(ok))
                    .map_err(|err| Forbidden(Some(err.json()))),
                Err(_) => Err(Forbidden(None)),
            })
            .await
        }
        _ => Err(Forbidden(None)),
    }
}*/

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("user mapping", |rocket| async {
        rocket.mount(
            "/user",
            routes![
                connect,
                connect_auto,
                disconnect,
                update,
                del,
                add,
                see_all_user,
                see_one_user,
                see_me,
                actualise,
                remove,
                //get_info
            ],
        )
    })
}
