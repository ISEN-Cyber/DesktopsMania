use super::schema::DataBase;
use super::AdHoc;
use super::HashMap;
use super::Template;
mod formulaire;
mod get;
mod set;
use formulaire::*;
use google_authenticator::ErrorCorrectionLevel;
use google_authenticator::GoogleAuthenticator;
use rocket::response::Flash;
use rocket::response::Redirect;

#[get("/temporaire/<path>")]
async fn display_temp(path: String) -> Template {
    let google_authenticator = GoogleAuthenticator::new();
    let secret = google_authenticator.create_secret(32);
    let image = google_authenticator.qr_code(
        secret.as_str(),
        "Desktop mania",
        "secure connexion",
        200,
        200,
        ErrorCorrectionLevel::High,
    ).unwrap();
    Template::render(
        "password",
        HashMap::from([
            ("path".to_string(), path),
            ("image".to_string(), image),
            ("secret".to_string(), secret),
        ]),
    )
}

#[post("/add/<path>", data = "<pass>")]
async fn post_temp(pass: PasswordForm, path: String, db: DataBase) -> Flash<Redirect> {
    match (pass.password(), pass.code(),pass.secret()) {
        (Ok(password), Ok(code),Ok(secret)) => {
            db.run(
                move |conn| match set::password(path, password, code,secret, conn) {
                    Ok(()) => Flash::success(
                        Redirect::to(uri!(super::index)),
                        "good, you can connect you",
                    ),
                    Err(()) => Flash::error(
                        Redirect::to(uri!(super::index)),
                        "sorry, don't work (and reask the admin to have a user profile)",
                    ),
                },
            )
            .await
        }
        _ => Flash::error(Redirect::to(uri!(super::index)), "you are week"),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("extra mapping", |rocket| async {
        rocket.mount("/extra", routes![display_temp, post_temp])
    })
}
