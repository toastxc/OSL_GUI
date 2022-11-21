#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



mod com;
use com::*;

pub mod structs {
    pub mod connection;
    pub mod details;
}


// internal crates
use crate::structs::{details::Details, connection::TokenResponse};
use crate::structs::connection::Connected;
use crate::structs::connection::{Build, BuildStream};
use crate::structs::connection::ProductFileResponse;



#[tauri::command]
async fn connect() -> String {


    let url = "http://buildservice.api.minalogger.com".to_string();
    let cres = osl_connect(url).await;

    let c = match cres {
        Err(e) => e.to_string(),
        Ok(c)  => format!("Version: {}\nAuthentication: {}\nProvider: {}",
            c.version, c.authprovider, c.authprovidersignup),
    };



    c





}


#[tauri::command]
async fn get() -> String {



    "success".to_string()


}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
