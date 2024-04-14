use defining_default_trait::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Select box draw");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 32,
                height: 32,
                options: vec![String::from("test")],
            }),
            Box::new(Button {
                width: 15,
                height: 32,
                label: String::from("Button"),
            }),
        ],
    };

    screen.run();
}
