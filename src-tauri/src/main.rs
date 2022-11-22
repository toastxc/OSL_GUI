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

    write_rel(res.clone());
   
    if res.len() == 0 {
        return "No products found!".to_string()
    
    }else if res.len() == 1 {
        return format!("Successfully saved one product");
    
    }else {

        return format!("Successfully saved {} products", res.len())
    };

}

#[tauri::command]
async fn products() -> String {

    let res = read_rel();

    if res.len() == 0 {
        return format!("No products!");
   
    };

    let mut returner = String::new();
    for x in 0..res.len() {
        returner += &format!("{returner}\n{}", res[x].productname);
    };
    println!("{:?}", res);

    returner 
}

#[tauri::command]
async fn info(products: String) -> String {

    let res = read_rel();

    if res.len() == 0 {
        return format!("No products!");

    };


    if products == "" {
        return format!("insert product name");
    };
       

    for x in 0..res.len() {
        if res[x].productname == products {
            return format!("{:?}", res[x]);
        };
    };

    return "No match found!".to_string()


}

// not sure what to do for installl



#[tauri::command]
async fn redeem(key: String) -> String {

    if key == "" {
        return format!("Insert key");
    };

    let auth = details_deser();

    let res = osl_redeem(key, auth).await;

    return res


}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect, get, products, info, redeem])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
