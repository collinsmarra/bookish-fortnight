use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::errors::{HTTPErrors, error_handler};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Task{
      pub completed_at: Option<String>,
      pub description: Option<String>,
      pub id: u32,
      pub priority: char,
      pub title: String,
    
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TaskResponse{
    pub data: Vec<Task>,
}

pub async fn get_all_tasks(token: String) -> Result<TaskResponse, HTTPErrors>{
    let response = Request::get("http://localhost:3000/api/v1/tasks")
        .header("X-Auth-Token", &*token)
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();

    match response.ok(){
        true => Ok(response.json::<TaskResponse>().await.unwrap()),
        false => Err(error_handler(response.status()))
    }

}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ApiCreateTaskResponseData{
    pub data: Task
}

pub async fn create_task(token: String, description: String, 
                         title: String, priority: char) -> Result<ApiCreateTaskResponseData, HTTPErrors>{
    let body = json!({
        "description": description,
        "title": title,
        "priority": priority
    });
    let response = Request::post("http://localhost:3000/api/v1/tasks")
        .header("X-Auth-Token", &*token)
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap();

    match response.ok(){
        true => Ok(response.json::<ApiCreateTaskResponseData>().await.unwrap()),
        false => Err(error_handler(response.status()))
    }
}

pub async fn get_task(token: String, id: u32) -> Result<ApiCreateTaskResponseData, HTTPErrors>{
    let response = Request::get(&*format!("http://localhost:3000/api/v1/tasks/{}", id))
        .header("X-Auth-Token", &*token)
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();
        
    match response.ok(){
        true => Ok(response.json::<ApiCreateTaskResponseData>().await.unwrap()),
        false => Err(error_handler(response.status()))
    }

}

pub async fn _edit_task(token: String,
                       task_id: u32, 
                       description: String,
                       priority: char, 
                       completed_at: String) -> Result<(), HTTPErrors>{
    
    let body = json!({
        "description": description,
        "prioroty": priority,
        "completed_at": completed_at
    });
    
    let response = Request::patch(&*format!("http://localhost:3000/api/v1/tasks/{}", task_id))
            .header("X-Auth-Token", &*token)
            .header("Content-Type", "applications/json")
            .body(body.to_string())
            .send()
            .await
            .unwrap();
    match response.ok(){
        true => Ok(()),
        false => Err(error_handler(response.status())),
    }
}

pub async fn api_delete_task(token: String, task_id: u32) -> Result<(), HTTPErrors>{
    let response = Request::delete(&*format!("http://localhost:3000/api/v1/tasks/{}",task_id))
            .header("X-Auth-Token", &*token)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

    match response.ok(){
        true => Ok(()),
        false => Err(error_handler(response.status())),
    }
}

pub async fn complete_task(token: String, task_id: u32) -> Result<(), HTTPErrors>{
    let response = Request::put(&*format!("http://localhost:3000/api/v1/tasks/{}/completed", task_id))
            .header("X-Auth-Token", &*token)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
    
    match response.ok(){
        true => Ok(()),
        false => Err(error_handler(response.status()))
    }
}

pub async fn uncomplete_task(token: String, task_id: u32) -> Result<(), HTTPErrors>{
    let response = Request::put(&*format!("http://localhost:3000/api/v1/tasks/{}/uncompleted", task_id))
            .header("X-Auth-Token", &*token)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();
    
    match response.ok(){
        true => Ok(()),
        false => Err(error_handler(response.status()))
    }
}
