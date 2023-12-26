use yew::prelude::*;

// let myValues: String = "ROYGBIV".to_string();
const SAMPLE: &str = "ROYGBIV";
const MATRIX_WIDTH:  usize = 1 + 1 + 1 + 7;
const MATRIX_HEIGHT: usize = 1 + 7;

enum Vitality {
    ALIVE,
    DEAD,
}

enum PlayerGroup {
    IMPOSTER,
    CREWMATE,
}

enum CellType {
    GroupCell( PlayerGroup ),
    TextCell( String ),
    VitalityCell( Vitality ),
}

impl CellType {

    // fn toggle_group(&self) -> CellType {
    //     match self.group {
    //         PlayerGroup::IMPOSTER => CellType::GroupCell { group: PlayerGroup::CREWMATE },
    //         PlayerGroup::CREWMATE => CellType::GroupCell { group: PlayerGroup::IMPOSTER },
    //         _ => todo!("Need to use PlayerGroup Cell!"),
    //     }
    // }
    //
    // fn toggle_vitality(&self) -> CellType {
    //     match self.state {
    //         Vitality::DEAD => CellType::VitalityCell { state: Vitality::ALIVE },
    //         Vitality::ALIVE => CellType::VitalityCell { state: Vitality::DEAD },
    //         _ => todo!("Need to use VitalityCell!"),
    //     }
    // }

    //NOTE: We will consume this value and create a new instance
    fn toggle(self) -> CellType {

        match self {
            CellType::GroupCell(PlayerGroup::IMPOSTER) => CellType::GroupCell(PlayerGroup::CREWMATE),
            CellType::GroupCell(PlayerGroup::CREWMATE) => CellType::GroupCell(PlayerGroup::IMPOSTER),
            CellType::VitalityCell(Vitality::ALIVE) => CellType::VitalityCell(Vitality::DEAD),
            CellType::VitalityCell(Vitality::DEAD) => CellType::VitalityCell(Vitality::ALIVE),
            _ => todo!("Need to implement a panic!"),
        }

    }

}

// struct Cell {
//     content: Option<String>,
//     cell_type: CellType,
//
// }
//
// struct Implicatory {
//     content: Option<String>,
// }
//
// struct Liveness {
//     content: Option<String>,
// }
//
// struct Identifier {
//     content: Option<String>,
// }
//
// struct Innocence {
//     content: Option<String>,
// }
//
//
// trait ClickBehavior {
//     fn click();
// }

struct AccusationMatrix {
    matrix: [[Cell; MATRIX_WIDTH]; MATRIX_HEIGHT],
}

impl AccusationMatrix {

    // fn new() -> AccusationMatrix {
    //
    // }

    // #[function_component(AccMatrix)]
    // fn display(&self) -> Html {
    // }
}

// enum PlayerState {
//     KILL,
//     LIVE,
// }

// Cell in the AccusationMatrix
struct Cell<T> {
    content: Option<String>,
    cell_type: Option<T>,
}


impl Cell {

    fn display(self) -> Html {
        html! {
            <td>{self.content.unwrap()}</td>
        }
    }

}

// fn populate_accusees() {
//     
// }
//
//
// fn populate_accusers() -> Result<bool, Err>{
//
// }


#[function_component(Table)]
fn table() -> Html {
    let test_header: String = "   ROYGBIV".to_string();
    let test_data01: String = "  R       ".to_string();
    let test_data02: String = "  O       ".to_string();
    let test_data03: String = "  Y       ".to_string();
    let test_data04: String = "  G       ".to_string();
    let test_data05: String = "  B       ".to_string();
    let test_data06: String = "  I       ".to_string();
    let test_data07: String = "  V       ".to_string();

    html! {
    <>
    <table>
    <tr>
    {
        for test_header.to_string()
            .chars()
            .map(|c| html!{<th>{c}</th>})
    }
    </tr>

    <tr>
    {
        for test_data01.to_string()
            .chars()
            .map(|c| html!{<td>{c}</td>})
    }
    </tr>
    </table>
    </>
        // <>
        // <table>
        // <tr>
        // <th>{"D"}</th>
        // <th>{"I?"}</th>
        // <th>{"C"}</th>
        // <th>{"R"}</th>
        // <th>{"O"}</th>
        // <th>{"Y"}</th>
        // <th>{"G"}</th>
        // <th>{"B"}</th>
        // <th>{"I"}</th>
        // <th>{"V"}</th>
        // </tr>
        //
        // <tr>
        // <td>{"D"}</td>
        // <td>{"I?"}</td>
        // <td>{"C"}</td>
        // <td>{"R"}</td>
        // <td>{"O"}</td>
        // <td>{"Y"}</td>
        // <td>{"G"}</td>
        // <td>{"B"}</td>
        // <td>{"I"}</td>
        // <td>{"V"}</td>
        // </tr>
        // </table>
        // </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <h1>{"Hello World!"}</h1>
        <Table />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
