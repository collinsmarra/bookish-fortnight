use std::ops::Deref;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use stylist::{Style, yew::styled_component};


use crate::store::TokenStore;
use crate::api::login::{api_login, api_signup, api_logout};
use crate::router::Route;

const STYLE_FILE: &str = include_str!("./static/login.css");


#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AuthDetails{
    pub username: String,
    pub password: String,
}


#[styled_component(SignUp)]
pub fn crate_account() -> Html{

    let state = use_state(|| AuthDetails::default());
    let is_auth = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let (_storage, auth_dispatch) = use_store::<TokenStore>();

    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let username_onchange = {
        let state = state.clone();
        Callback::from( move| event: Event|{
            let username = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut data = state.deref().clone();
            data.username = username;
            state.set(data);
        })
    };
    
    let password_onchange = {
        let state = state.clone();
        Callback::from( move| event: Event|{
            let password = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut data = state.deref().clone();
            data.password = password;
            state.set(data);
        })
    };

    let handle_form_onsubmit = {
        let is_auth = is_auth.clone();
        let navigator = navigator.clone();
        Callback::from( move|event: SubmitEvent|{
            event.prevent_default();
            let dispatch = auth_dispatch.clone();
            let username = state.username.clone();
            let password = state.password.clone();
            let is_auth = is_auth.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async  move{
                let response = api_signup(username, password).await;
                let token = response.token;
                if token != ""{
                    // flash message
                    dispatch.reduce_mut(|store| store.token = token);
                    is_auth.set(true);
                    navigator.push(&Route::Home);
                }
            })
        })
    };
    
    html!{
        <div class={stylesheet}>
            <form onsubmit={handle_form_onsubmit} class="form">
                <div>
                   <div>
                        <label for="username">{"Username"}</label>
                   </div> 
                   <div>
                        <input type="text" id="username" class="username" placeholder="username" onchange={username_onchange}/>
                   </div> 
                   <div>
                        <label for="password" class="label">{"Password"}</label>
                   </div> 
                   <div>
                        <input type="password" id="password" class="username" placeholder="password" onchange={password_onchange}/>
                   </div> 
                   <div></div>
                   <div class="butt">
                        <button>{"Sign Up"}</button>
                   </div> 
                </div>
            </form>
        </div>
    }
}


#[styled_component(Login)]
pub fn login() -> Html{
    let state = use_state(|| AuthDetails::default());
    let is_auth = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let (_storage, auth_dispatch) = use_store::<TokenStore>();

    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let username_onchange = {
        let state = state.clone();
        Callback::from( move| event: Event|{
            let username = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut data = state.deref().clone();
            data.username = username;
            state.set(data);
        })
    };

    let password_onchange = {
        let state = state.clone();
        Callback::from( move| event: Event|{
            let password = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut data = state.deref().clone();
            data.password = password;
            state.set(data);
        })
    };

    let handle_form_onsubmit = {
        let is_auth = is_auth.clone();
        let navigator = navigator.clone();
        Callback::from( move|event: SubmitEvent|{
            event.prevent_default();
            let dispatch = auth_dispatch.clone();
            let username = state.username.clone();
            let password = state.password.clone();
            let is_auth = is_auth.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async  move{
                let response = api_login(username, password).await;
                let token = response.token;
                if token != ""{
                    // flash message
                    dispatch.reduce_mut(|store| store.token = token);
                    is_auth.set(true);
                    navigator.push(&Route::Home);
                }
            })
        })
    };

    html!{
        <div class={stylesheet}>
            <form onsubmit={handle_form_onsubmit} class="form">
                <div>
                   <div>
                        <label for="username">{"Username"}</label>
                   </div> 
                   <div>
                        <input type="text" id="username" class="username" placeholder="username" onchange={username_onchange}/>
                   </div> 
                   <div>
                        <label for="password" class="label">{"Password"}</label>
                   </div> 
                   <div>
                        <input type="password" id="password" class="username" placeholder="password" onchange={password_onchange}/>
                   </div> 
                   <div></div>
                   <div class="butt">
                        <button>{"Login"}</button>
                   </div> 
                </div>
            </form>
        </div>
    }

}

#[function_component(LogOut)]
pub fn logout() -> Html{
    let token = use_selector(|store: &TokenStore| store.token.clone());
    let (_storage, auth_dispatch) = use_store::<TokenStore>();
    let navigator = use_navigator().unwrap();


    /*
     *
    let handle_logout = {
        let navigator = navigator.clone();
        let token = token.clone();
        let dispatch = auth_dispatch.clone();
        Callback::from(move |_event: MouseEvent|{
            let navigator = navigator.clone();
            let token = token.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = api_logout(token.deref().clone()).await;
                match response{
                    Ok(_) => {
                        dispatch.reduce_mut(|store| store.token = "".to_owned());
                        navigator.push(&Route::Home);
                    },
                    Err(_) => {
                        navigator.push(&Route::Home);
                    }
                }
            })
        })
    };
    *
    */
    {
        let navigator = navigator.clone();
        let token = token.clone();
        let dispatch = auth_dispatch.clone();
        use_effect_with_deps(move |_| {
            let navigator = navigator.clone();
            let token = token.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move{
                let response = api_logout(token.deref().clone()).await;
                match response{
                    Ok(_) => {
                        dispatch.reduce_mut(|store| store.token = "".to_owned());
                        navigator.push(&Route::Home);
                    },
                    Err(_) => {
                        navigator.push(&Route::Home);
                    }
                }
            });
            || ()
        }, ());
    };

    html!{
        <div>
        </div>
    }
}
