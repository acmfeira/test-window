#![windows_subsystem = "windows"]
/*
    mostra primeira tela, ao sair do foco, conta 5 segundos e aparece a segunda, ocultando a primeira...
    NAO FUNCIONA NO RWINDOWS..... MAS FUNCIONA BEM NO LINUX :(
    JA PROCUREI VARIAS OPCOES E NADA!!!!!!
*/
use std::{thread, time::Duration};

use fltk::{app, enums::Event, prelude::*, window::Window};
// use fltk::enums::{Event, WindowFlag};

fn main() {
    let app = app::App::default();
    
    // Create the main window
    let mut main_win = Window::new(100, 100, 500, 600, "Main Window");
    main_win.end();
    main_win.show();

    // Create the second window
    let mut second_win = Window::new(200, 200, 300, 300, "Second Window");
    second_win.end();
    
    // Set the second window as always on top
    // second_win.set_flag(WindowFlag::AlwaysOnTop, true);

    // Start with the second window hidden
    second_win.hide();

    // Add a handler to the main window to handle minimizing
    main_win.handle(move |win, ev| match ev {
        Event::Unfocus => {
            // When main window loses focus (minimized), show the second window
            // second_win.make_resizable(true);
            // second_win.set_override();
            thread::sleep(Duration::from_secs(5));
            second_win.show();
            // second_win.make_current();

            win.hide();
            // second_win.make_current();
            // second_win.set_override();
            // second_win.set_visible_focus();
            // second_win.show();
            true
        }
        _ => false,
    });

    app.run().unwrap();
}
