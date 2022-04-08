use serde::{Deserialize, Serialize};
use sudoku::Sudoku;
use yew::prelude::*;

enum Msg {
    SetValue(usize, usize),
}

#[derive(Serialize, Deserialize)]
struct Model {
    pub sudoku: String,
}

fn update_sudoku(sudoku: String, idx: usize, guess: usize) -> Sudoku {
    let new: String = sudoku
        .chars()
        .enumerate()
        .map(|(i, g)| {
            if i == idx {
                char::from_digit(guess as u32, 10).expect("")
            } else {
                g
            }
        })
        .collect();
    Sudoku::from_str_line(&new).expect("")
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            sudoku: Sudoku::generate_unique().to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetValue(idx, guess) => {
                self.sudoku = update_sudoku(self.sudoku.clone(), idx, guess).to_string();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <p>{ self.sudoku.clone() }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
