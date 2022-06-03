
use rocket::form::Form;
use super::super::message::Message;

#[derive(FromForm)]
pub struct CreateVM {
    #[field(name = "name",validate=len(2..64))]
    name: String,
    #[field(name = "os",validate=len(2..64))]
    os: String,
}

impl CreateVM {
    pub fn name(&self) -> Result<String, Message> {
        Ok(self.name.to_string())
    }
    pub fn os(&self) -> Result<String, Message> {
        Ok(self.os.to_string())
    }
}
pub type CreateVMForm = Form<CreateVM>;


#[derive(FromForm)]
pub struct SelectVM {
    #[field(name = "title",validate=len(2..64))]
    name: String,
}

impl SelectVM {
    pub fn name(&self) -> Result<String, Message> {
        Ok(self.name.to_string())
    }
}
pub type SelectVMForm = Form<SelectVM>;


#[derive(FromForm)]
pub struct UserAddVM {
    #[field(name = "title",validate=len(2..64))]
    name: String,
    #[field(name = "email",validate=len(2..64))]
    email: String,
}

impl UserAddVM {
    pub fn name(&self) -> Result<String, Message> {
        Ok(self.name.to_string())
    }
    pub fn email(&self) -> Result<String, Message> {
        Ok(self.email.to_string())
    }
}
pub type UserAddVMForm = Form<UserAddVM>;