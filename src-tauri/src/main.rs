#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod com;
use com::*;
mod fs;
use fs::*;
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

    let auth = details_deser();

    let cres = osl_connect(auth.url).await;

    let c = match cres {
        Err(e) => e.to_string(),
        Ok(c)  => format!("Version: {}\nAuthentication: {}\nProvider: {}",
            c.version, c.authprovider, c.authprovidersignup),
    };
    c


}


#[tauri::command]
async fn get() -> String {


    let auth = details_deser();

    let res = osl_release(auth).await;

    println!("{:?}", res);

    let mut returner = String::new();
   
    if res.len() == 0 {
        return "No products found!".to_string()
    };
    for x in 0..res.len() {

        returner = format!("returner {:?}", res[x]);
    };

    return returner

}





fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
