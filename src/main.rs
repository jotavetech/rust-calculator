use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry};

fn main() {
    let app = Application::builder()
        .application_id("org.jotavetech.Calculator")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(360)
            .default_height(500)
            .title("The Best Calculator Ever!")
            .build();

        let vbox = Box::new(gtk::Orientation::Vertical, 0);
        win.add(&vbox);

        let input = gtk::Entry::builder().height_request(100).build();
        vbox.add(&input);

        let grid = gtk::Grid::builder()
            .margin(10)
            .column_spacing(5)
            .row_spacing(5)
            .build();

        for (i, _button) in (1..=10).enumerate() {
            let button = button_builder(&i.to_string(), &input);

            grid.attach(&button, i as i32 % 3, i as i32 / 3, 1, 1);
        }

        let equal_button = button_builder("=", &input);
        let clear_button = button_builder("C", &input);
        let plus_button = button_builder("+", &input);
        let minus_button = button_builder("-", &input);
        let multiply_button = button_builder("*", &input);
        let divide_button = button_builder("/", &input);

        grid.attach(&equal_button, 3, 3, 1, 1);
        grid.attach(&clear_button, 3, 0, 1, 1);
        grid.attach(&divide_button, 3, 1, 1, 1);
        grid.attach(&multiply_button, 3, 2, 1, 1);
        grid.attach(&plus_button, 1, 3, 1, 1);
        grid.attach(&minus_button, 2, 3, 1, 1);

        vbox.add(&grid);

        win.show_all();
    });

    app.run();
}

fn button_builder(label: &str, input: &Entry) -> Button {
    let button = Button::with_label(label);
    button.set_vexpand(true);
    button.set_hexpand(true);

    let label_cloned = label.to_string();
    let input_cloned = input.clone();

    button.connect_clicked(move |_| get_button_input(&label_cloned, &input_cloned));

    button
}

fn get_button_input(value: &str, input: &Entry) {
    let current_text = input.text();
    let new_text = format!("{}{}", current_text, value);
    input.set_text(&new_text);

    eprintln!("Button clicked! {}", value);
}
