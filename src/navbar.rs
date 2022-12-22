use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use std::ops::Deref;
use stylist::{Style, yew::styled_component};


use crate::router::Route;
use crate::store::TokenStore;

const STYLE_FILE: &str = include_str!("./static/navbar.css");

#[styled_component(Navbar)]
pub fn navbar() -> Html{
    // let (store, dispatch) = use_selector::<TokenStore>();
    let _token = use_selector(|store: &TokenStore| store.token.clone());

    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let _onclick_logout = {
        Callback::from(move| event: Event|{
            event.prevent_default();
            todo!();
        })
    };

    html!{
        <div class={stylesheet}>
            <nav>
                <ul>
                    <li>
                        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Tasks}>{"Tasks"}</Link<Route>>
                    </li>
                    {
                        if _token.is_empty(){
                            html!{
                                <li>
                                    <Link<Route> to={Route::Auth}>{"Login"}</Link<Route>>
                                </li>
                            }
                        }else{
                            html!{}
                        }
                    }
                    {
                        if _token.is_empty(){
                            html!{
                                <li>
                                    <Link<Route> to={Route::SignUp}>{"Register"}</Link<Route>>
                                </li>
                            }
                        }else{
                            html!{}
                        }
                    }
    
                    <li>
                        // <a href="#" onclick={_onclick_logout}>{"Logout"}</a>
                        <Link<Route> to={Route::LogOut}>{"Logout"}</Link<Route>>
                    </li>
                </ul>
            </nav>
        </div>
    }
}


fn _is_logged_in(token: String) -> bool{
    !token.deref().is_empty()
}
