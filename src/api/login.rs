use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::errors::{HTTPErrors, error_handler};


#[derive(Serialize, Deserialize)]
pub struct  AuthResponse{
    pub id: u32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiAuthResponseData{
    pub data: AuthResponse
}

#[derive(Serialize, Deserialize)]
pub struct ApiLogoutResponseData{
    pub message: String,
}


pub async fn api_signup(username: String, password: String) -> AuthResponse{
    let body = json!({
        "username": username,
        "password": password
    });
    let response = Request::post("http://localhost:3000/api/v1/users")
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiAuthResponseData>()
        .await.unwrap();
    
    response.data
}



pub async fn api_login(username: String, password: String) -> AuthResponse{
    let body = json!({
        "username": username,
        "password": password
    });
    let response = Request::post("http://localhost:3000/api/v1/users/login")
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiAuthResponseData>()
        .await.unwrap();
    
    response.data
}


#[allow(unused)]
pub async fn api_logout(token: String) -> Result<(), HTTPErrors>{
    let response = Request::post("http://localhost:3000/api/v1/users/logout")
            .header("content-type", "application/json")
            .header("x-auth-token", &*token)
            .send()
            .await
            .unwrap();

    match response.ok(){
        true => Ok(()),
        false => Err(error_handler(response.status()))
    }
}
