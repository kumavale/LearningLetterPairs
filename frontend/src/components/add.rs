use yew::prelude::*;

#[function_component(AddButton)]
pub fn add_button() -> Html {
    html! {
        <>
            <button class="fixed-button" type="button" data-bs-toggle="modal" data-bs-target="#add-modal">{"\u{FF0B}"}</button>
        </>
    }
}

#[function_component(AddModal)]
pub fn add_modal() -> Html {
    html! {
        <div class="modal fade" id="add-modal" tabindex="-1">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <form>
                        <div class="modal-header">
                            <h5 class="modal-title">{"Append Pair"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <div class="mb-3">
                                <label for="InputPair" class="form-label">{"Pair"}</label>
                                <input type="text" class="form-control" id="InputPair" placeholder="AB"
                                    pattern=".{2,2}" title="Please input 2 characters." required=true />
                            </div>
                            <div class="mb-3">
                                <label for="InputObject" class="form-label">{"Object"}</label>
                                <input type="text" class="form-control" id="InputObject" placeholder="ABS"
                                    pattern=".{1,32}" title="Please input 32 characters or less." required=true />
                            </div>
                            <div class="mb-3">
                                <label for="InputImage" class="form-label">{"Image"}</label>
                                <input class="form-control" type="file" id="InputImage" accept="image/*" aria-describedby="imageHelp" />
                                <div id="imageHelp" class="form-text">{"Images are cropped to a maximum of 256x256."}</div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                            <button type="submit" class="btn btn-primary">{"Save changes"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
