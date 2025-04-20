use leptos::prelude::*;
use reactive_stores::Store;

use crate::app::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn DifficultySlider() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let difficulty = state.difficulty();

    view! {
        <div class="w-full max-w-xs">
        <label>
            <p>Reading Level</p>
            <input type="range" min="0" max="12" value="6" class="range" step="1" on:change:target=move |ev| {
                difficulty.set(ev.target().value().parse().unwrap())
            }/>
        </label>
        <div class="flex justify-between px-2.5 mt-2 text-xs">
            {
                (1..13).map(|i| view! { <span>{i}</span> }).collect_view()
            }
        </div>
        </div>
    }.into_any()
}
