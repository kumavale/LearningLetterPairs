use yew::prelude::*;

// TODO: モックを作成しただけ
#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="contents">
            <form class="form-signin">
                // Username input
                <div class="form-group mb-4">
                    <label class="form-label" for="username">{"Username"}</label>
                    <input type="text" id="input-username" class="form-control" placeholder="Username" required=true />
                </div>
                // Password input
                <div class="form-group mb-4">
                    <label class="form-label" for="password">{"Password"}</label>
                    <input type="password" id="input-password" class="form-control" placeholder="Password" required=true />
                </div>
                // 2 column grid layout for inline styling
                <div class="row mb-4">
                    <div class="col d-flex justify-content-center">
                        // Checkbox
                        <div class="form-check">
                            <label class="form-check-label" for="input-remember" style="white-space: nowrap;">{"Remember me"}</label>
                            <input class="form-check-input" type="checkbox" value="" id="input-remember" checked=true />
                        </div>
                    </div>
                    <div class="col">
                        <a href="#!">{"Forgot password?"}</a>
                    </div>
                </div>
                // Submit button
                <button type="submit" class="btn btn-primary btn-block mb-4" style="width: 100%;">{"Sign in"}</button>
                // Register buttons
                <div class="text-center">
                    <p>{"Not a member? "}<a href="#!">{"Register"}</a></p>
                </div>
            </form>
        </div>
    }
}
