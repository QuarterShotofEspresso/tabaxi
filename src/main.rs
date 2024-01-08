use dioxus::{prelude::*};

const CREWMATES: usize = 13;
const TOTAL_COLUMNS: usize = CREWMATES + 1;
const TOTAL_ROWS_WO_ACCUSEE:    usize = CREWMATES + 1;

const IMG_PATHS: [&'static str; CREWMATES] = [
    "cmicons/black.png",
    "cmicons/cyan.png",
    "cmicons/red.png",
    "cmicons/white.png",
    "cmicons/gray.png",
    "cmicons/orange.png",
    "cmicons/yellow.png",
    "cmicons/pink.png",
    "cmicons/blue.png",
    "cmicons/brown.png",
    "cmicons/lime.png",
    "cmicons/purple.png",
    "cmicons/green.png",
];

enum PlayerGroup {
    IMPOSTER,
    CREWMATE,
    UNKNOWN,
}

fn tick_player_group(group: &PlayerGroup) -> PlayerGroup {
    match group {
        PlayerGroup::UNKNOWN => PlayerGroup::IMPOSTER,
        PlayerGroup::IMPOSTER => PlayerGroup::CREWMATE,
        PlayerGroup::CREWMATE => PlayerGroup::UNKNOWN,
    }
}



enum Vitality {
    ALIVE,
    DEAD,
}

fn tick_vitality(state: &Vitality) -> Vitality {
    match state {
        Vitality::ALIVE => Vitality::DEAD,
        Vitality::DEAD  => Vitality::ALIVE,
    }
}


fn main() {
    dioxus_web::launch(app);
}


fn spacer(cx: Scope) -> Element {
    render! {
        div {
            border: "1px solid black",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            height: "75px",
            "spacer"
        },
    }
}

fn intro(cx: Scope) -> Element {

    render! {
        div {
            font_family: "Luminari",
            font_size: "50px",
            border: "1px solid black",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            text_align: "center",
            "Tabaxi"
        },
    }
}


#[derive(Props, PartialEq)]
struct AccuserRowProps {
    row_idx: usize
}


fn accuser_row(cx: Scope<AccuserRowProps>) -> Element {
    render! {
        tr {
            (0..TOTAL_COLUMNS).map(|i| match i {
                0 => rsx! { crewmate_cell { path: IMG_PATHS[cx.props.row_idx - 1] }},
                _ => rsx! { implicatory_cell {} }
            })
        }
    }
}

fn accusee_row(cx: Scope) -> Element {
    render! {
        tr {
            (0..TOTAL_COLUMNS).map(|i| match i {
                0 => rsx! { empty_cell {} },
                _ => rsx! { crewmate_cell { path: IMG_PATHS[i - 1] } },
            })
        }
    }
}

#[derive(Props, PartialEq)]
struct ImageCellProps<'a> {
    path: &'a str 
}

fn crewmate_cell<'a>(cx: Scope<'a, ImageCellProps<'a>>) -> Element {
    let liveness = use_state(cx, || Vitality::ALIVE);
    let background_color = use_state(cx, || "white".to_string());

    render! {
        match liveness.get() {
            Vitality::ALIVE => rsx! ( td {
                onmouseenter: move |_| background_color.set("lightgray".to_string()),
                onmouseleave: move |_| background_color.set("white".to_string()),
                onclick: move |_| liveness.set(tick_vitality(liveness.get())),
                background_color: "{background_color}",
                img { src: cx.props.path },
            }),
            Vitality::DEAD => rsx! ( td {
                onmouseenter: move |_| background_color.set("lightgray".to_string()),
                onmouseleave: move |_| background_color.set("gray".to_string()),
                onclick: move |_| liveness.set(tick_vitality(liveness.get())),
                background_color: "{background_color}",
                img { src: cx.props.path },
            }),
        }
    }
}

fn empty_cell(cx: Scope) -> Element {
    render! {
        td { "" }
    }
}


fn implicatory_cell(cx: Scope) -> Element {
    let curr_group = use_state(cx, || PlayerGroup::UNKNOWN);
    let background_color = use_state(cx, || "white".to_string());

    render! {
        match curr_group.get() {
            PlayerGroup::UNKNOWN => rsx! ( td {
                onmouseenter: move |_| background_color.set("pink".to_string()),
                onmouseleave: move |_| background_color.set("white".to_string()),
                onclick: move |_| curr_group.set(tick_player_group(curr_group.get())),
                background_color: "{background_color}"
            }),
            PlayerGroup::IMPOSTER => rsx! ( td {
                onmouseenter: move |_| background_color.set("cyan".to_string()),
                onmouseleave: move |_| background_color.set("red".to_string()),
                onclick: move |_| curr_group.set(tick_player_group(curr_group.get())),
                background_color: "{background_color}"
            }),
            PlayerGroup::CREWMATE => rsx! ( td {
                onmouseenter: move |_| background_color.set("gray".to_string()),
                onmouseleave: move |_| background_color.set("blue".to_string()),
                onclick: move |_| curr_group.set(tick_player_group(curr_group.get())),
                background_color: "{background_color}"
            }),
        }
    }
}


fn implication_matrix(cx:Scope) -> Element {
    render! {
        div {
            border: "1px solid black",
            display: "flex",
            justify_content: "center",
            align_items: "center",

            table {
                accusee_row(cx),
                (1..TOTAL_ROWS_WO_ACCUSEE).map( |i| rsx!{accuser_row{ row_idx: i }} )
            }
        }
    }
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    render! {
        spacer(cx),
        intro(cx),
        spacer(cx),
        implication_matrix(cx),
    }
}
