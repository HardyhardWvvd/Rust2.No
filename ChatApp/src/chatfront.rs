//This is a test code for creating the front end using Rust and its dependencies. It has not been used anywhere
use yew:prelude::*;

struct Model{
    value: i64
}

#[function_component()]
fn app() -> Html{
    let state = use_state(||Model{
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_|{
            state.set(Model{
                value: state.value+1
            })
        })
    };
    html!{
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{state.value}</p>
        </div>
    }
}
fn main(){
    yew::start_app::<app>();
}