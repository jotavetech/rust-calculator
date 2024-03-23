use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};

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
            let button = button_builder(&i.to_string());
            button.connect_clicked(move |_| eprintln!("Button clicked! {}", &i));

            grid.attach(&button, i as i32 % 3, i as i32 / 3, 1, 1);
        }

        let equal_button = button_builder("=");
        let clear_button = button_builder("C");
        let plus_button = button_builder("+");
        let minus_button = button_builder("-");
        let multiply_button = button_builder("*");
        let divide_button = button_builder("/");

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

fn button_builder(label: &str) -> Button {
    let button = Button::with_label(label);
    button.set_vexpand(true);
    button.set_hexpand(true);

    button
}
