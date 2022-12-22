use yew::prelude::*;
use yew_router::prelude::*;

mod store;
mod api;
mod auth;
mod router;
mod tasks;
mod navbar;
mod flash;
mod errors;

use router::{Route, switch};


#[function_component(App)]
pub fn view() -> Html{
    let say_hi = use_state(||true);
    let say_hi_clone = say_hi.clone();
    gloo::timers::callback::Timeout::new(5000, move||{
        say_hi_clone.set(false);
    }).forget();

    html!{

        <BrowserRouter>
            <div>
                <navbar::Navbar />
                <h1>{"App"}</h1>
                if *say_hi{
                    <p>{"Hello world"}</p>
                }
            <Switch<Route> render={switch}/>
        </div>
        </BrowserRouter>
    }
}
