use slint::Weak;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

slint::slint!{
    import { VerticalBox } from "std-widgets.slint";

    export global CalcLogic {
        callback button-pressed(string);
    }

    component Button {
        in property <string> text;
        min-height: 30px;
        min-width: 30px;
        in property <brush> background: #1d78d3;
        Rectangle {
            background: ta.pressed ? red : ta.has-hover ? background.darker(0.1) : background;
            animate background { duration: 100ms; }
            border-radius: 4px;
            border-width: 2px;
            border-color: self.background.darker(0.2);
            ta := TouchArea {
                clicked => {CalcLogic.button-pressed(root.text);}
            }
        }
        Text { text: root.text; }
    }

    export component App inherits Window {
        in property <int> value: 1;
        GridLayout {
            padding: 10px;
            spacing: 5px;
            Text {  text:  value; colspan: 3; }
            Row {
                Button { text: "1"; }
                Button { text: "2"; }
                Button { text: "3"; }
                Button { text: "+"; background: lightblue; }
            }
            Row {
                Button { text: "4"; }
                Button { text: "5"; }
                Button { text: "6"; }
                Button { text: "-"; background: lightblue; }
            }
            Row {
                Button { text: "7"; }
                Button { text: "8"; }
                Button { text: "9"; }
                Button { text: "*"; background: lightblue; }
            }
            Row {
                Button { text: "0"; }
                Button { text: "="; col: 2; background: lightblue;}
                Button { text: "/"; background: lightblue;}
            }

        }
    }
}

#[derive(Default)]
struct CalcState {
    prev_value: i32,
    current_value: i32,
    operator: slint::SharedString,
}

fn main() {
    let app: App = App::new().unwrap();
    let weak: Weak<App> = app.as_weak();
    let state: Rc<RefCell<CalcState>> = Rc::new(RefCell::new(CalcState::default()));
    app.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let mut state: RefMut<CalcState> = state.borrow_mut();
        if let Ok(val) = value.parse::<i32>() {
            state.current_value *= 10;
            state.current_value += val;
            app.set_value(state.current_value);
            return;
        }
        match value.as_str() {
            "+" => {
                state.prev_value = state.current_value;
                state.current_value = 0;
                state.operator = "+".into();
            }
            "-" => {
                state.prev_value = state.current_value;
                state.current_value = 0;
                state.operator = "-".into();
            }
            "*" => {
                state.prev_value = state.current_value;
                state.current_value = 0;
                state.operator = "*".into();
            }
            "/" => {
                state.prev_value = state.current_value;
                state.current_value = 0;
                state.operator = "/".into();
            }
            "=" => {
                match state.operator.as_str() {
                    "+" => {
                        state.current_value = state.prev_value + state.current_value;
                    }
                    "-" => {
                        state.current_value = state.prev_value - state.current_value;
                    }
                    "*" => {
                        state.current_value = state.prev_value * state.current_value;
                    }
                    "/" => {
                        state.current_value = state.prev_value / state.current_value;
                    }
                    _ => {}
                }
                app.set_value(state.current_value);
            }
            _ => {}
            
        }
    });
    
    app.run().unwrap();
    println!("Hello, world!");
}
