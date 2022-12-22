use yew_router::prelude::*;
use yew::prelude::*;
use crate::auth::{SignUp, Login, LogOut};
use crate::tasks::{CreateTask, ViewTasks, ViewTask, DeleteTask};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route{
    #[at("/")]
    Home,
    #[at("/login")]
    Auth,
    #[at("/register")]
    SignUp,
    #[at("/logout")]
    LogOut,
    #[at("/tasks")]
    Tasks,
    #[at("/tasks/create")]
    CreateTask,
    #[at("/tasks/view/:id")]
    ViewTask {id: u32},
    #[at("/tasks/delete/:id")]
    DeleteTask {id: u32},
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html{
    match routes{
        Route::Home => html!{<h1>{ "Home"}</h1>},
        Route::Auth => html!{<Login />},
        Route::SignUp => html!{<SignUp />},
        Route::LogOut => html!{<LogOut />},
        Route::Tasks => html!{<ViewTasks />},
        Route::CreateTask => html!{<CreateTask />},
        Route::ViewTask{ id:_ } => html!{<ViewTask />},
        Route::DeleteTask{ id:_ } => html!{<DeleteTask />},
        Route::NotFound => html!{<h1>{"404"}</h1>}
    }
}
