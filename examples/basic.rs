use iui::prelude::*;
use iui_builder::iui;

fn main() {
    // Initialize the UI library and create the main window
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);

    // Create the layout and add the controls
    iui! { &ui,
        let contents = VerticalBox(padded: true) {
            Stretchy: let _lbl = Label("\
                There is a ton of text in this label.\n\
                Pretty much every unicode character is supported.\n\
                🎉 用户界面 사용자 인터페이스\n\
            ")
            Compact: let _grp = Group("Group") {
                let _vbx = VerticalBox() {
                    Compact: let button = Button("Button")
                    Compact: let quit_button = Button("Quit")
                }
            }
        }
    }

    // Assign a handler to button clicks
    button.on_clicked(&ui, |btn| {
        btn.set_text(&ui, "Clicked!");
    });
    quit_button.on_clicked(&ui, |_| {
        ui.quit();
    });

    // Put the layout in the window, show the window, and run the application
    win.set_child(&ui, contents);
    win.show(&ui);
    ui.main();
}
