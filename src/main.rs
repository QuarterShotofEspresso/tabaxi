use dioxus::prelude::*;

// mod img_path {
//     pub const BLACK:  &str = "cmicons/black.png";
//     pub const CYAN:   &str = "cmicons/cyan.png";
//     pub const RED:    &str = "cmicons/red.png";
//     pub const WHITE:  &str = "cmicons/white.png";
//     pub const GRAY:   &str = "cmicons/gray.png";
//     pub const ORANGE: &str = "cmicons/orange.png";
//     pub const YELLOW: &str = "cmicons/yellow.png";
//     pub const PINK:   &str = "cmicons/pink.png";
//     pub const ROSE:   &str = "cmicons/rose.png";
//     pub const BROWN:  &str = "cmicons/brown.png";
//     pub const LIME:   &str = "cmicons/lime.png";
//     pub const PURPLE: &str = "cmicons/purple.png";
//     pub const GREEN:  &str = "cmicons/green.png";
// }
//

const CREWMATES: usize = 13;

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


fn im_row(cx: Scope<ImageCellProps>) -> Element {
    render! {
        tr {
            // (0..13).map(|i| match i {
            //         0 => rsx!{ im_image_cell { id: cx.props.id } },
            //         _ => rsx!{ im_todo_cell {} }
            //     })
            (0..CREWMATES).map(|i| match i {
                0 => rsx! { im_image_cell { id: cx.props.id }},
                _ => rsx! { im_todo_cell {} }
            })
            // IMG_PATHS.iter().map( |&path| rsx! {
            //     td {
            //         img {
            //             width: "20px",
            //             height: "22px",
            //             src: path,
            //         }
            //     }
            // })
        }
    }
}

fn im_header(cx: Scope) -> Element {
    render! {
        tr {
            IMG_PATHS.iter().map( |&path| rsx!{
                th { 
                    img { 
                        width: "20px",
                        height: "22px",
                        src: path, 
                    } 
                } 
            })
        }
    }
}


#[derive(Props, PartialEq)]
struct ImageCellProps {
    id: usize 
}

// #[derive(Props, PartialEq)]
// struct ImageCellProps<'a> {
//     id: &'a i32 
// }

fn im_image_cell(cx: Scope<ImageCellProps>) -> Element {
    render! {
        td {
            // "{cx.props.id}"
            img {
                width: "20px",
                height: "22px",
                src: IMG_PATHS[cx.props.id]
            }
            // image: "cmicons/cyan.png"
        }
    }
}

// fn im_image_cell<'a>(cx: Scope<'a, ImageCellProps<'a>>) -> Element {
//     render! {
//         "{cx.props.id}"
//     }
// }

fn im_todo_cell(cx: Scope) -> Element {
    render! {
        td { "" }
    }
}


fn im(cx:Scope) -> Element {
    render! {
        div {
            border: "1px solid black",
            display: "flex",
            justify_content: "center",
            align_items: "center",

            table {
                im_header(cx),
                (0..CREWMATES).map( |i| rsx!{im_row{ id: i }} )
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
        im(cx),
    }
}
