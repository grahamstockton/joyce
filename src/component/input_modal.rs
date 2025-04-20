use leptos::{html::Dialog, prelude::*, task::spawn_local};
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields as _},
    component::difficulty_slider::DifficultySlider,
};

#[component]
pub fn InputModal() -> impl IntoView {
    // state signals
    let state = expect_context::<Store<GlobalState>>();
    let difficulty = state.difficulty();
    let input = state.input_str();
    let output = state.output_str();

    // ref to close dialog
    let e = NodeRef::<Dialog>::new();

    view! {
        <button class="btn" onclick="my_modal_1.showModal()">Input Text</button>
        <dialog node_ref=e id="my_modal_1" class="modal">
        <div class="modal-box">
            <h3 class="text-lg font-bold pb-2">Input text</h3>
            <textarea placeholder="Neutral" class="textarea textarea-neutral w-full"  on:change:target=move |ev| {input.set(ev.target().value())}></textarea>
            <div class="modal-action mt-4 pb-2">
            <form method="dialog" class="w-full">
                <DifficultySlider />
                <div class="flex pt-3 space-x-2">
                    <div class="btn btn-primary" on:click=move |_| { spawn_local(async move {
                        // call llm and set response if successful
                        let _ = match get_llm_response(difficulty.get(), input.get()).await {
                            Ok(s) => output.set(s),
                            Err(_) => output.set("Error calling llm. Please try again later.".to_string()),
                        };

                        // close modal
                        e.get().unwrap().close();
                    })}>Submit</div>
                    <button class="btn">Close</button>
                </div>
            </form>
            </div>
        </div>
        </dialog>
    }.into_any()
}

#[server]
async fn get_llm_response(difficulty: i32, input: String) -> Result<String, ServerFnError> {
    use crate::llm::llm_wrapper::LlmWrapper;

    match use_context::<LlmWrapper>() {
        Some(llm) => llm
            .get_output(difficulty, input)
            .await
            .map_err(|e| ServerFnError::new(format!("error calling llm: {:?}", e))),
        None => Err(ServerFnError::new("No llm client found")),
    }
}
