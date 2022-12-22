use yew::prelude::*;
use yewdux::prelude::*;



use crate::store::TokenStore;

#[derive(Clone, Debug, PartialEq)]
pub struct Task{
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props{
    tasks: Vec<Task>,
}


#[function_component(TaskProps)]
pub fn task_props(props: &Props) -> Html{
    let (_storage, dispatch) = use_store::<TokenStore>();
    let tasks = props.tasks.clone();

    tasks.iter().map(|task|{
        html!{
            <div>
                <h1>{&task.title}</h1>
                <p>{&task.description.as_ref().unwrap_or(&"No description".to_string())}</p>
            </div>
        }
    }).collect::<Html>()
}

