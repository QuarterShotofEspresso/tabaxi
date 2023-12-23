use yew::prelude::*;


#[function_component(Table)]
fn table() -> Html {
    html! {
        <table>
        <tr>
        <th>D</th>
        <th>I?</th>
        <th>C</th>
        <th>R</th>
        <th>O</th>
        <th>Y</th>
        <th>G</th>
        <th>B</th>
        <th>I</th>
        <th>V</th>
        </tr>

        <tr>
        <td>D</td>
        <td>I?</td>
        <td>C</td>
        <td>R</td>
        <td>O</td>
        <td>Y</td>
        <td>G</td>
        <td>B</td>
        <td>I</td>
        <td>V</td>
        </tr>
        </table>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
            <h1>{"Hello World!"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
