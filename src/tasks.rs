use yew::prelude::*;
use yew_router::prelude::use_location;
use yew_router::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlInputElement;
use gloo::console::log;
use std::ops::Deref;
use stylist::{Style, yew::styled_component};

use crate::store::TokenStore;

use crate::api::tasks::{get_all_tasks, TaskResponse, create_task, Task,
    ApiCreateTaskResponseData, get_task};


use crate::api::tasks;
use crate::{flash, flash::MessageType};
use crate::router::Route;

const STYLE_FILE: &str = include_str!("./static/task_improve.css");
const TASK_STYLE_FILE: &str = include_str!("./static/task-form.css");

#[styled_component(ViewTasks)]
pub fn view() -> Html{
    let token = use_selector(|state: &TokenStore| state.token.clone());
    let tasks = use_state(|| TaskResponse::default());
    let (_storage, auth_dispatch) = use_store::<TokenStore>();

    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let stylesheet = stylesheet.clone();
    
    {
        let tasks = tasks.clone();
        use_effect_with_deps(move |_|{
            let tasks = tasks.clone();
            let token = token.clone();
            let dispatch = auth_dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = get_all_tasks(token.deref().clone()).await;
                // log!(serde_json::to_string_pretty(&response).unwrap());
                match response{
                    Ok(response) => {
                        let mut data = tasks.deref().clone();
                        data.data = response.data.clone();
                        dispatch.reduce_mut(|store| store.tasks = response.data);
                        tasks.set(data);
                    },
                    Err(_error) => {
                        log!("You are not logged in".to_string());
                    }
                }
            });
            || ()
        }, ());
    };

    html!{
        <div class={stylesheet.clone()}>
            <h1>{"Tasks"}</h1>
            <div class="task">
                <div class="task-priority">{"Priority"}</div>
                <div class="task-checkbox">{"Completed"}</div>
                <div class="task-title">{"Title"}</div>
            </div>

            {
                tasks.data.deref().into_iter().map(|task|{
                    html!{
                        <div>
                            <div class="task">
                                <div class="task-priority">{&task.priority}</div>
                                {   
                                    if task.completed_at.is_some(){
                                        html!{
                                            <div class="task-checkbox">
                                                <input type="checkbox" checked=true/>
                                            </div>
                                        }
                                    }else{
                                        html!{
                                            <div class="task-checkbox">
                                                <input type="checkbox"/>
                                            </div>
                                        }
                                    }
                                }
                                <div class="task-title"> <Link<Route> to={Route::ViewTask {id: task.id.clone()}}>{&task.title}</Link<Route>> </div>
                            </div>
                        </div>
                    }}).collect::<Html>()
            }
        </div>
    }
}


#[styled_component(CreateTask)]
pub fn create() -> Html{
    let token = use_selector(|state: &TokenStore| state.token.clone());
    let task = use_state(|| Task::default());
    let task_response = use_state(|| ApiCreateTaskResponseData::default());

    let stylesheet = Style::new(TASK_STYLE_FILE).unwrap();

    let navigator = use_navigator().unwrap();

    let handle_form_onsubmit = {
        let task = task.clone();
        let task_response = task_response.clone();
        Callback::from(move |event: SubmitEvent|{
            event.prevent_default();
            let (token, navigator) = (token.deref().clone(), navigator.clone());
            let title = task.title.clone();
            let description = task.description.clone().unwrap();
            let priority = task.priority.clone();
            let task_response = task_response.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = create_task(token, description, title, priority).await;
                match response{
                    Ok(response) => {
                        let mut new_data = task_response.deref().clone();
                        new_data.data = response.data;
                        let id = new_data.data.id;
                        task_response.set(new_data);
                        gloo::timers::callback::Timeout::new(2000, move||{
                            navigator.push(&Route::ViewTask { id: id });
                            
                        }).forget();
                    },
                    Err(_err) =>{
                        log!("You need to be logged in".to_string());
                    }
                }
            })
        })
    };
    let onchange_title = {
        let task = task.clone();
        Callback::from(move |event: Event|{
            let title = event
                .target_unchecked_into::<HtmlInputElement>()
                .value();
            let mut data = task.deref().clone();
            data.title = title;
            task.set(data);
        })
    };

    let onchange_description = {
        let task = task.clone();
        Callback::from(move |event: Event|{
            let description = event
                .target_unchecked_into::<HtmlInputElement>()
                .value();
            let mut data = task.deref().clone();
            data.description = Some(description);
            task.set(data);
        })
    };

    let onchange_priority = {
        let task = task.clone();
        Callback::from(move| event: Event|{
            let priority = event
                .target_unchecked_into::<HtmlInputElement>()
                .value();
            let mut data = task.deref().clone();
            data.priority = priority.chars().next().unwrap();
            task.set(data);
        })
    };

    let _onchange_completed_at = {
        // date type
        let task = task.clone();
        Callback::from(move |event: Event|{
            let completed_at = event
                .target_unchecked_into::<HtmlInputElement>()
                .value();
            let mut data = task.deref().clone();
            data.completed_at = Some(completed_at);
            task.set(data);
        })
    };

    html!{
        <div class={stylesheet}>
            <h1>{"Set task"}</h1>
            <form onsubmit={handle_form_onsubmit}>
                <div>
                    <label>{"Title"}</label>
                </div>
                <div>
                    <input type="text" name="title" onchange={onchange_title}/>
                </div>
                <div>
                    <label>{"Priority"}</label>
                </div>
                <div>
                    // <input type="text" name="priority" />
                    <select namenone="priority" onchange={onchange_priority}>
                        <option value="A" selected={true}>{"A"}</option>
                        <option value="B" >{"B"}</option>
                        <option value="C" >{"C"}</option>
                        <option value="D" >{"D"}</option>
                    </select>
                </div>
                <div>
                    <label>{"Description"}</label>
                </div>
                <div>
                    <textarea name="description" onchange={onchange_description} placeholder="Enter description here ...">
                    </textarea>
                </div>
                // <div>
                //     <label>{"DeadLine"}</label>
                // </div>
                // <div>
                //     <input type="date" onchange={onchange_completed_at}/>
                // </div>
                <div>
                    <button>{"Create task"}</button>
                </div>
                // {navigator.push(&Route::ViewTask { id: task_response.deref().clone().data.id })}
            </form>
        </div>
    }
}


#[styled_component(ViewTask)]
pub fn view_task() -> Html{

    let token = use_selector(|state: &TokenStore| state.token.clone());
    
    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();
    let task_response = use_state(|| ApiCreateTaskResponseData::default());

    let unused_token = token.clone();
    
    let stylesheet = Style::new(STYLE_FILE).unwrap();


    {
        let task_response = task_response.clone();
        let (location, navigator) = (location.clone(), navigator.clone());
        let path: Vec<&str> = location.path().split("/").collect::<Vec<&str>>().clone();
        let task_id = path[path.len() -1].parse::<u32>().unwrap_or_default();
        use_effect_with_deps(move |_|{
            let task_response = task_response.clone();
            let token = token.clone();
            let (task_id, navigator) = (task_id.clone(), navigator.clone());
            wasm_bindgen_futures::spawn_local(async move {
                let response = get_task(token.deref().clone(), task_id).await;

                match response {
                    Ok(response) => {
                        let mut new_data = task_response.deref().clone();
                        new_data.data = response.data;
                        task_response.set(new_data);
                    },
                    Err(_err) => {
                        log!("Task not found".to_string());
                        navigator.push(&Route::NotFound);
                    }
                }
                // let mut data = task_response.deref().clone();
                // data.data = response.data;
                // task_response.set(data);
            });
            || ()
        }, ());
    };

    let oncomplete_task ={
        let location = location.clone();
        let navigator = navigator.clone();
        let path: Vec<&str> = location.path().split("/").collect::<Vec<&str>>().clone();
        let task_id = path[path.len() -1].parse::<u32>().unwrap_or_default();
        let unused_token = unused_token.clone();
        Callback::from(move |event: MouseEvent|{
            event.prevent_default();
            let unused_token = unused_token.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = tasks::complete_task(unused_token.deref().clone(), task_id).await;
                match response{
                    Ok(_) => {
                        navigator.push(&Route::ViewTask { id: task_id });
                    },
                    Err(_) => {}
                }
            })
        })
    };

    let onuncomplete_task ={
        let location = location.clone();
        let navigator = navigator.clone();
        let path: Vec<&str> = location.path().split("/").collect::<Vec<&str>>().clone();
        let task_id = path[path.len() -1].parse::<u32>().unwrap_or_default();
        let unused_token = unused_token.clone();
        Callback::from(move |event: MouseEvent|{
            event.prevent_default();
            let unused_token = unused_token.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = tasks::uncomplete_task(unused_token.deref().clone(), task_id).await;
                match response{
                    Ok(_) => {
                        navigator.push(&Route::ViewTask { id: task_id });
                    },
                    Err(_) => {}
                }
            })
        })
    };



    html!{
        <div class={stylesheet}>
            <h1>{"View task"}</h1>
            // <button onclick={onclick}>{"Get task"}</button>
            <div>
                // <CreateTask />
                <div>
                    <div class="button-container">
                        <Link<Route> to={Route::CreateTask}>
                            <button class="creation">{"Create Task"}</button>
                        </Link<Route>>
                        <DeleteTask/>
                    </div>
                </div>
            </div>
            <div >
                <div class="task">
                    <div class="task-priority">{"Priority"}</div>
                    <div class="task-checkbox">{"Completed"}</div>
                    <div class="task-title">{"Title"}</div>
                </div>
                <div class="task">
                    <div class="task-priority">{&task_response.deref().data.priority}</div>
                    {  
                        if task_response.deref().data.completed_at.is_some(){
                            html!{
                                <div class="task-checkbox" onclick={onuncomplete_task}>
                                    <input type="checkbox" checked={true}/>
                                </div>
                            }
                        }else{
                            html!{
                                <div class="task-checkbox" onclick={oncomplete_task}>
                                    <input type="checkbox" checked={false}/>
                                </div>
                            }
                        }
                    }
                    <div class="task-title">{&task_response.deref().data.title}</div>
                </div>
            </div>
        </div>
    }
}

const BUTTON_STYLE_FILE: &str = include_str!("./static/button.css");

#[styled_component(DeleteTask)]
pub fn delete() -> Html{
    let token = use_selector(|store: &TokenStore| store.token.clone());
    let location = use_location().unwrap();
    let success_message = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());


    let navigator = use_navigator().unwrap();
    let stylesheet = Style::new(BUTTON_STYLE_FILE).unwrap();

    let onclick_delete = {
        let token = token.clone();
        let location = location.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();
        let path: Vec<&str> = location.path().split("/").collect::<Vec<&str>>().clone();
        let task_id = path[path.len() -1].parse::<u32>().unwrap_or_default();
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent|{
            let token = token.clone();
            let success_message = success_message.clone();
            let error_message = error_message.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = tasks::api_delete_task(token.deref().clone(), task_id).await;
                match response {
                    Ok(_) => {
                        let message = "Task Deleted successfully".to_string();
                        success_message.set(message);
                        gloo::timers::callback::Timeout::new(5000, move||{
                            success_message.set("".to_string());
                            navigator.push(&Route::Tasks);
                        }).forget();
                    },
                    Err(_err) => {
                        let message = "Error in deleting task".to_string();
                        error_message.set(message);
                        gloo::timers::callback::Timeout::new(8000, move||{
                            error_message.set("".to_string());
                        }).forget();
                    }
                }
            })
        })
    };

    log!(&success_message.deref().clone());
    log!(&error_message.deref().clone());
    html!{
        <div class={stylesheet}>
            // <h1>{"Delete task"}</h1>
            {
                if success_message.deref().len() > 0 {
                    html!{
                        <div class="sucess">
                            <flash::FlashMessage message_type={MessageType::Success(success_message.deref().clone())} />
                            // <Link<Route> to={Route::Tasks} >{"Go to tasks"}</Link<Route>>
                            // {navigator.push(&Route::Tasks)}
                        </div>
                    }
                } else { html!{} }
            }
        {
            if error_message.deref().len() > 0{
                html!{
                    <div class="error">
                        <flash::FlashMessage message_type={MessageType::ErrorMessage(error_message.deref().clone())} />
                    </div>
                }
            }else{ html!{} }
        }
            <div class="divbutt"><button class="delete" onclick={onclick_delete.clone()}>{"Delete task"}</button></div>
        </div>
    }
}


#[styled_component(EditTask)]
pub fn edit_task() -> Html{
    let stylesheet = Style::new(BUTTON_STYLE_FILE).unwrap();

    html!{
        <div class={stylesheet}>
            <div class="divbutt">
                <button class="edit">{"Edit Task"}</button>
            </div>
        </div>
    }
}
