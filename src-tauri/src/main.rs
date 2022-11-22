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
async fn streams(va: String) -> String {

    if va == "" {
        return format!("Data required");
    };

    let mut va_vec: Vec<&str> =  va.split('/').collect::<Vec<&str>>();


    // structure 
    // product, stream

    let release = read_rel();


       let auth = details_deser();

    if va_vec.len() > 0 {
        for x in 0..release.len() {

            if release[x].productname == va_vec[0] {

                let product = &release[x];

                let mut stre = String::new();
            
                for y in 0..product.streams.len() {
                    stre += &format!("{}", product.streams[y].branchname);
                };

                if va_vec.len() < 2 {
                    return format!("Available streams: {}", stre);
                };

                println!("\n\n\n{:?}", product);

                for z in 0..product.streams.len() {
                    
                    if product.streams[z].branchname == va_vec[1] {

   
                        let hash = &product.streams[z].commithash;

                        println!("{hash}");

                        let streamf = osl_file(auth.url.clone(), hash.to_string(), auth.token.clone()).await;

                        println!("{:?}", streamf);

                        let mut greer = String::new();

                        for p in 0..streamf.len() {

                            greer += &format!("\n{}", streamf[p].location);
                        

                        };

                        return greer

                    };
                };
            };
        };
    };

    String::new()
}




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
        .invoke_handler(tauri::generate_handler![connect, get, products, info, redeem, streams])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
