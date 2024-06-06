extern crate termion;

mod menu {
    use std::i32;
    use std::io::{self, stdin, stdout};
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;

    pub struct MenuItem {
        pub name: String,
        pub value: String,
    }

    pub struct Menu {
        pub items: Vec<MenuItem>,
    }

    impl Drop for Menu {
        fn drop(&mut self) {
            // Turn the cursor back on
            print!("\x1B[?25h");
        }
    }

    impl Menu {
        pub fn new() -> Self {
            Menu { items: Vec::new() }
        }

        pub fn add_item(&mut self, name: String, value: String) {
            self.items.push(MenuItem { name, value });
        }

        fn draw_menu_options(&mut self, redraw: bool, counter: i32) {
            if redraw {
                print!("\x1B[{}A", (self.items.len()) as i32);
            }

            for (idx, item) in self.items.iter().enumerate() {
                let terminator = if idx == self.items.len() { "" } else { "\n" };
                let cursor = if idx as i32 == counter { "> " } else { "  " };

                // TODO: Move the color codes to a string builder
                print!(
                    "\x1B[38;5;122m{}\x1B[38;5;15m {}{}",
                    cursor, item.name, terminator
                )
            }
        }

        fn get_key_input() -> Key {
            let _stdout = stdout().into_raw_mode().unwrap();

            let key = stdin().keys().next().unwrap().unwrap();

            let stdout = io::stdout();
            let stdout = stdout.lock();
            let _ = stdout.into_raw_mode();
            key
        }

        pub fn print(&mut self) {
            // Turn the cursor off
            print!("\x1B[?25l");

            let mut counter = 0;
            let len = self.items.len() as i32;

            self.draw_menu_options(false, counter);

            loop {
                let key = Menu::get_key_input();

                match key {
                    Key::Char('q') => break,
                    Key::Char('w') | Key::Up => {
                        counter = (counter + len - 1) % len;
                        self.draw_menu_options(true, counter)
                    }
                    Key::Char('s') | Key::Down => {
                        counter = (counter + 1) % len;
                        self.draw_menu_options(true, counter);
                    }
                    Key::Char('\n') => {
                        let item = &self.items[counter as usize];
                        println!("You chose {}", item.value);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    println!("Please chose an option: ");
    let mut menu = menu::Menu::new();

    menu.add_item(String::from("Option 1"), String::from("Value 1"));
    menu.add_item(String::from("Option 2"), String::from("Value 2"));
    menu.add_item(String::from("Option 3"), String::from("Value 3"));

    menu.print();
}
