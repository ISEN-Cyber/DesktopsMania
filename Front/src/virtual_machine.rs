use super::message::Message;
use super::message::MessageJson;
use super::schema::DataBase;
use super::session;
use super::AdHoc;
use formulaire::*;
use rocket::http::CookieJar;
use rocket::response::status::Forbidden;
mod formulaire;
mod get;
mod set;
use session::*;
use rocket::serde::json::Json;

#[post("/link/<path>",data = "<link_vm>")]
async fn link(path: String,link_vm:SelectVMForm,  cookies: &CookieJar<'_>, db: DataBase) -> Result<String, Forbidden<String>> {
    match (cookies.get_private("session"),link_vm.name())
    {
       (Some(cookie),Ok(name)) => {
           let data = cookie.value().to_string();
           db
               .run(move |conn| match Session::get_session(data, path, conn)
           {
               Ok(session) => get::vemmion(name,session.get_reference(),conn).map_err(|_|Forbidden(None)),
               Err(_) => Err(Forbidden(None)),
           }).await
       }
       _ => Err(Forbidden(None)),
   }
}

#[post("/add_user/<path>",data = "<useradd>")]
async fn add_user(path: String,useradd:UserAddVMForm,  cookies: &CookieJar<'_>, db: DataBase) -> Result<MessageJson, Forbidden<String>> {
    match (cookies.get_private("session"),useradd.name(),useradd.email())
    {
       (Some(cookie),Ok(name),Ok(email)) => {
           let data = cookie.value().to_string();
           db
               .run(move |conn| match Session::get_session_by_level(data, path,LEVEL::ADMIN, conn)
           {
               Ok(session) => Ok(set::associate(name,email,session.get_reference(),conn)),
               Err(_) => Err(Forbidden(None)),
           }).await
       }
       _ => Err(Forbidden(None)),
   }
}

#[post("/count/<path>",data = "<link_vm>")]
async fn count(path: String,link_vm:SelectVMForm,  cookies: &CookieJar<'_>, db: DataBase) -> Result<String, Forbidden<String>> {
    match (cookies.get_private("session"),link_vm.name())
    {
       (Some(cookie),Ok(name)) => {
           let data = cookie.value().to_string();
           db
               .run(move |conn| match Session::get_session(data, path, conn)
           {
               Ok(_) => Ok(get::user_on_vm_juste_chiffre(name,conn).to_string()),
               Err(_) => Err(Forbidden(None)),
           }).await
       }
       _ => Err(Forbidden(None)),
   }
}

#[post("/user/<path>",data = "<link_vm>")]
async fn user(path: String,link_vm:SelectVMForm,  cookies: &CookieJar<'_>, db: DataBase) -> Result<Json<Vec<String>>, Forbidden<String>> {
    match (cookies.get_private("session"),link_vm.name())
    {
       (Some(cookie),Ok(name)) => {
           let data = cookie.value().to_string();
           db
               .run(move |conn| match Session::get_session_by_level(data, path,LEVEL::ADMIN, conn)
           {
               Ok(session) => Ok(Json(get::user_on_vm_for_admin(session.get_reference(),name,conn))),
               Err(_) => Err(Forbidden(None)),
           }).await
       }
       _ => Err(Forbidden(None)),
   }
}


#[post("/remove_user/<path>",data = "<useradd>")]
async fn remove_user(path: String,useradd:UserAddVMForm,  cookies: &CookieJar<'_>, db: DataBase) -> Result<MessageJson, Forbidden<String>> {
    match (cookies.get_private("session"),useradd.name(),useradd.email())
    {
       (Some(cookie),Ok(name),Ok(email)) => {
           let data = cookie.value().to_string();
           db
               .run(move |conn| match Session::get_session_by_level(data, path,LEVEL::ADMIN, conn)
           {
               Ok(session) => Ok(set::remove_associate(name,email,session.get_reference(),conn)),
               Err(_) => Err(Forbidden(None)),
           }).await
       }
       _ => Err(Forbidden(None)),
   }
}


#[post("/off/<path>",data = "<link_vm>")]
async fn off(path: String, link_vm:SelectVMForm,cookies: &CookieJar<'_>, db: DataBase) -> Result<MessageJson, Forbidden<String>> {
    match (cookies.get_private("session"),link_vm.name()) {
        (Some(cookie),Ok(name)) => {
            let data = cookie.value().to_string();
            let clone = name.clone();
            match db
                .run(move |conn| Session::get_session(data, path, conn))
                .await
            {
                Ok(session) => match {
                    let yop = session.get_reference();
                    db.run(move |conn| set::get_vm(name, yop, conn))
                        .await
                } {
                    Ok(link) => match get::stop_machine(link).await {
                        Ok(_) => {
                            db.run(move |conn| {
                                Ok(set::ip_stop(clone.to_string(), session.get_reference(), conn))
                            })
                            .await
                        }
                        Err(_) => Err(Forbidden(None)),
                    },
                    Err(_) => Err(Forbidden(Some("retry later".to_string()))),
                },
                Err(_) => Err(Forbidden(None)),
            }
        }
        _ => Err(Forbidden(None)),
    }
}

#[post("/on/<path>",data = "<link_vm>")]
async fn on(
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
    link_vm:SelectVMForm
) -> Result<MessageJson, Forbidden<String>> {
    match (cookies.get_private("session"),link_vm.name()) {
        (Some(cookie),Ok(name)) => {
            let data = cookie.value().to_string();
            let clone = name.to_string();
            match db
                .run(move |conn| Session::get_session(data, path, conn))
                .await
            {
                Ok(session) => match {
                    let yop = session.get_reference();
                    db.run(move |conn| set::get_vm(name.to_string(), yop, conn))
                        .await
                } {
                    Ok(link) => match get::start_machine(link).await {
                        Ok(ip) => {
                            
                            db.run(move |conn| {
                                Ok(set::ip(clone.to_string(), ip, session.get_reference(), conn))
                            })
                            .await
                        }
                        Err(_) => Err(Forbidden(None)),
                    },
                    Err(_) => Err(Forbidden(Some("retry later".to_string()))),
                },
                Err(_) => Err(Forbidden(None)),
            }
        }
        _ => Err(Forbidden(None)),
    }
}

#[post("/delete/<path>", data = "<delete_vm>")]
async fn del(
    delete_vm: SelectVMForm,
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> Result<MessageJson, Forbidden<String>> {
    match (cookies.get_private("session"), delete_vm.name()) {
        (Some(cookie), Ok(name)) => {
            let data = cookie.value().to_string();
            match db
                .run(move |conn| Session::get_session_by_level(data, path, LEVEL::ADMIN, conn))
                .await
            {
                Ok(session) => match db
                    .run(move |conn| {
                        set::remove_vm(name.to_string(), session.get_reference(), conn)
                    })
                    .await
                {
                    Ok(link) => Ok(get::delete_machine(link)
                        .await
                        .map_or_else(|_| Message::unknow(), |_| Message::unknow())
                        .json()),
                    Err(_) => Err(Forbidden(Some("retry later".to_string()))),
                },
                Err(_) => Err(Forbidden(None)),
            }
        }
        _ => Err(Forbidden(None)),
    }
}
#[post("/add/<path>", data = "<create_vm>")]
async fn add(
    create_vm: CreateVMForm,
    path: String,
    cookies: &CookieJar<'_>,
    db: DataBase,
) -> Result<MessageJson, Forbidden<String>> {
    match (
        cookies.get_private("session"),
        create_vm.name(),
        create_vm.os(),
    ) {
        (Some(cookie), Ok(name), Ok(os)) => {
            let data = cookie.value().to_string();
            match db
                .run(move |conn| Session::get_session_by_level(data, path, LEVEL::ADMIN, conn))
                .await
            {
                Ok(session) => match get::create_machine(
                    os.to_string(),
                    name.to_string(),
                    session.get_reference(),
                )
                .await
                {
                    Ok(link) => {
                        db.run(move |conn| {
                            Ok(set::add_vm(
                                name.to_string(),
                                session.get_reference(),
                                link,
                                conn,
                            ))
                        })
                        .await
                    }
                    Err(_) => Err(Forbidden(Some("retry later".to_string()))),
                },
                Err(_) => Err(Forbidden(None)),
            }
        }
        _ => Err(Forbidden(None)),
    }
}

#[get("/all/<path>")]
async fn get_all(path: String, cookies: &CookieJar<'_>, db: DataBase) -> Result<Json<Vec<get::VirtualMachine>>, MessageJson> {
    match cookies.get_private("session")
     {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db
                .run(move |conn| match Session::get_session(data, path, conn)
            {
                Ok(session) => get::you(session.get_reference(),conn).map(|ok| Json(ok)).map_err(|e|e.json()),
                Err(_) => Err(Message::unknow().json()),
            }).await
        }
        _ => Err(Message::unknow().json()),
    }
}

#[get("/<path>")]
async fn get_my(path: String, cookies: &CookieJar<'_>, db: DataBase) -> Result<Json<Vec<get::VirtualMachine>>, MessageJson> {
    match cookies.get_private("session")
     {
        Some(cookie) => {
            let data = cookie.value().to_string();
            db
                .run(move |conn| match Session::get_session(data, path, conn)
            {
                Ok(session) => get::me(session.get_reference(),conn).map(|ok| Json(ok)).map_err(|e|e.json()),
                Err(_) => Err(Message::unknow().json()),
            }).await
        }
        _ => Err(Message::unknow().json()),
    }
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("virtual machine mapping", |rocket| async {
        rocket.mount("/virtual_machine", routes![user,count,remove_user,add_user,link, off, on, del, add,get_all,get_my])
    })
}
