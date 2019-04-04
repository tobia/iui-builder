use iui::prelude::*;
use iui_builder::iui;

fn main() {
    // Initialize the UI library and create the main window
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Test App", 400, 200, WindowType::NoMenubar);

    // Create the layout and add the controls
    iui! { &ui,
        let contents = TabGroup() {
            ("Checkbox", margined: true): let _vbx = VerticalBox() {
                Compact: let _ctl = Checkbox("Option one", checked: true)
                Compact: let _ctl = Checkbox("Option two")
                Compact: let _ctl = Checkbox("Option three")
            }
            ("Combobox", margined: true): let _vbx = VerticalBox() {
                Compact: let _ctl = Combobox(selected: 1) {
                    "Option one", "Option two", "Option three"
                }
            }
        }
    }

    // Put the layout in the window, show the window, and run the application
    win.set_child(&ui, contents);
    win.show(&ui);
    ui.main();
}
