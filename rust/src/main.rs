#![allow(dead_code, unused_variables)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate log;
extern crate fern;
extern crate rouler;

use crate::controllers::dice_service;

mod models;
mod controllers;

fn main() {
    env_logger::init();
    
    rocket::ignite()
        .mount("/api/diceservice", routes![
            dice_service::dice,
            dice_service::ff_dices,
            dice_service::ff_dice_media])
        .launch();    
}