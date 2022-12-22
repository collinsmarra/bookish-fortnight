use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum MessageType{
    ErrorMessage(String),
    Success(String),
    Info(String),
}

impl Default for MessageType{
    fn default() -> Self{
        MessageType::Info(String::new())
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FlashMessageProps{
    pub message_type: Option<MessageType>,
}

#[function_component(FlashMessage)]
pub fn flash_message(props: &FlashMessageProps) -> Html{
    let message_type = props.message_type.clone();
    html!{
        <div class="flash-message">
            <Flash message_type={message_type} />
        </div>
    }
}

#[function_component(Flash)]
pub fn flash(props: &FlashMessageProps) -> Html{
    let message_type = props.message_type.clone();
    
    html!{
        <div>
            {
                match message_type{
                    Some(MessageType::ErrorMessage(message)) => {
                        html!{
                            <div class="error">
                                <p>{message}</p>
                            </div>
                        }
                    },
                    Some(MessageType::Success(message)) => {
                        html!{
                            <div class="success">
                                <p>{message}</p>
                            </div>
                        }
                    },
                    Some(MessageType::Info(message)) => {
                        html!{
                            <div class="info">
                                <p>{message}</p>
                            </div>
                        }
                    },
                    None => {
                        html!{}
                    }
                }
            }
        </div>
    }
}

