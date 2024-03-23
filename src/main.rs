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

        let input = gtk::Entry::builder()
            .placeholder_text("Digite algo maneiro amigao!")
            .height_request(50)
            .margin(10)
            .build();

        vbox.pack_start(&input, false, false, 0);
        vbox.set_child_packing(&input, false, true, 0, gtk::PackType::Start);

        let button = Button::with_label("Click me!");

        button.connect_clicked(|_| eprintln!("Clicked!"));

        // Add the button to the window.
        win.add(&button);

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}
