use dioxus::prelude::*;



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
            height: "250px",
            "spacer"
        },
    }
}

fn intro(cx: Scope) -> Element {

    render! {
        div {
            border: "1px solid black",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            text_align: "center",
            "TABAXI"
        },
    }

}


fn im_row(cx: Scope<ImageCellProps>) -> Element {
    render! {
        tr {
            // border: "1px solid black",
            // border_collapse: "collapse",
            (0..15).map(|i| match i {
                    // 0 => rsx!{ im_image_cell { id: &i } },
                    0 => rsx!{ im_image_cell { id: cx.props.id } },
                    _ => rsx!{ im_todo_cell {} }
                })
                // rsx!{ td {"{i}"} })
        }
    }
}

fn im_header(cx: Scope) -> Element {
    render! {
        tr {
            // border: "1px solid black",
            // border_collapse: "collapse",
            (0..15).map(|i| {
                rsx!{ 
                    th {
                        // border: "1px solid black",
                        // border_collapse: "collapse",
                        "{i}"
                    } 
                }
            })
        }
    }
}


#[derive(Props, PartialEq)]
struct ImageCellProps {
    id: i32 
}

// #[derive(Props, PartialEq)]
// struct ImageCellProps<'a> {
//     id: &'a i32 
// }

fn im_image_cell(cx: Scope<ImageCellProps>) -> Element {
    render! {
        td {
            // border: "1px solid black",
            // border_collapse: "collapse",
            "{cx.props.id}"
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
        td {
            // border: "1px solid black",
            // border_collapse: "collapse",
            ""
        }
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
                // border: "1px solid black",
                // border_collapse: "collapse",
                im_header(cx),
                (1..15).map(|i| rsx!{im_row{ id: i }} )
                // im_row,
                // im_row,
                // im_row,
                // im_row,
                // im_row,
                // im_row,
                // im_row,
                // im_row,
            }
        }
    }
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    render! {
        spacer(cx),
        intro(cx),
        im(cx),
    }
}
