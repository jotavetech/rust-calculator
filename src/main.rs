use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};

fn main() {
    let app = Application::builder()
        .application_id("org.jotavetech.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(360)
            .default_height(500)
            .title("Hello, World! Im Jotavetech")
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
            let button = Button::with_label(&i.to_string());
            button.set_vexpand(true);
            button.set_hexpand(true);
            button.connect_clicked(move |_| eprintln!("Button clicked! {}", &i));

            grid.attach(&button, i as i32 % 3, i as i32 / 3, 1, 1);
        }

        let equal_button = Button::with_label("=");
        equal_button.set_vexpand(true);
        equal_button.set_hexpand(true);

        let clear_button = Button::with_label("C");
        clear_button.set_vexpand(true);
        clear_button.set_hexpand(true);

        let plus_button = Button::with_label("+");
        plus_button.set_vexpand(true);
        plus_button.set_hexpand(true);

        let minus_button = Button::with_label("-");
        minus_button.set_vexpand(true);
        minus_button.set_hexpand(true);

        let multiply_button = Button::with_label("*");
        multiply_button.set_vexpand(true);
        multiply_button.set_hexpand(true);

        let divide_button = Button::with_label("/");
        divide_button.set_vexpand(true);
        divide_button.set_hexpand(true);

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
