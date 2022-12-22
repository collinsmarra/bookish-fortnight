//use crate::api::tasks::Task;
use yew::prelude::*;
use crate::api::tasks::TaskResponse;


#[function_component(SingleTaskHtmlView)]
pub fn single_task_view(data: &TaskResponse) -> Html{
    let task = data.data[0].clone();
    
    let completed_at = &task.completed_at.unwrap();
    let description = &task.description.unwrap();
    let id = &task.id;
    let priority = &task.priority;
    let title = &task.title;

    html!{
        <div class="task">
            <div class="task-title">{title}</div>
            <div class="task-description">{description}</div>
            <div class="task-description">{completed_at}</div>
            <div class="task-footer">
                <span class="task-priority">{priority}</span>
                <span class="task-id">{"Task #"} {id}</span>
            </div>
        </div>
    }
}
