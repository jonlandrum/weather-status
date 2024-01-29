use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Settings" }</h1>
            <fieldset>
                <legend>{ "System of Measurements" }</legend>
                <div>
                    <label for="metric">
                        <input type="radio" id="metric" name="system" value="metric" checked=true />
                        { "Metric" }
                    </label>
                </div>
                <div>
                    <label for="imperial">
                        <input type="radio" id="imperial" name="system" value="imperial" />
                        { "Imperial" }
                    </label>
                </div>
            </fieldset>
            <fieldset>
                <legend>{ "Location Coordinates" }</legend>
                <div>
                    <label for="latitude">
                        { "Latitude" }<br />
                        <input id="latitude" type="number" min="-90" max="90" step="any" value="0" />
                    </label>
                </div>
                <div>
                    <label for="longitude">
                        { "Longitude" }<br />
                        <input id="longitude" type="number" min="-180" max="180" step="any" value="0" />
                    </label>
                </div>
            </fieldset>
            <input type="submit" value="Save" />
        </main>
    }
}
