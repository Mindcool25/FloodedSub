#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};

pub mod data;
use crate::data::common::{SubsonicResponseWrapper, SubsonicResponse};
use crate::data::info::{License, OpenSubsonicExtensionList};

const SERVER_NAME: &str = "OpenSubServer";
const SERVER_VERSION: &str = "0.0.1 (alpha)";
const SUPPORTED_VERSION: &str = "1.16.1";



#[get("/ping")]
fn ping() -> Json<SubsonicResponseWrapper<()>> {
    return Json(SubsonicResponse::<()>::new().resp())
}

#[get("/getOpenSubsonicExtensions")]
fn get_extensions() -> Json<SubsonicResponseWrapper<OpenSubsonicExtensionList>> {
    return Json(SubsonicResponse::<OpenSubsonicExtensionList>::new().resp())
}

#[get("/getLicense")]
fn get_license() -> Json<SubsonicResponseWrapper<License>> {
    return Json(SubsonicResponse::<License>::new().resp())
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/rest", routes![ping,  get_extensions, get_license])
}
