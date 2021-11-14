extern crate log;

use std::env;
use std::path::Path;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket::get;
use rouler::Roller;
use serde::{Deserialize, Serialize};

/*
 * https://crates.io/crates/rouler
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct FfDicesApiResponse {
    dices: Vec<i64>,
}

#[get("/normal/<dice>")]
pub fn dice(dice: String) -> String {

    let stat = Roller::new(dice.as_str());
    format!("{}", stat.total())
}

#[get("/ff/dices/<dices>")]
pub fn ff_dices(dices: i64) -> Json<FfDicesApiResponse>  {

    let mut response = FfDicesApiResponse { dices: vec![], };

    let dice = "1d6";
    for _ in 0..dices {
        let stat = Roller::new(dice);
        response.dices.push(stat.total());
    }
    Json(response)    
}

#[get("/ff/dice/media")]
pub fn ff_dice_media() -> Option<NamedFile>  {

    let dice_response = dice(String::from("1d6"));
    let dice_name = format!("{}{}.png", "dice", dice_response);
    
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let media_dir = "/src/media/dices/ff/";
    let full_current_dir = current_dir + media_dir;

    let path_os_string = Path::new(full_current_dir.as_str()).join(dice_name);

    log::error!("{:?}", path_os_string);
    NamedFile::open(path_os_string).ok()
}
