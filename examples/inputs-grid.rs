use iui::prelude::*;
use iui_builder::iui;
use std::cell::RefCell;
use std::rc::Rc;

// This struct will hold the state of the application
struct State {
    slider_val: i64,
    spinner_val: i64,
    entry_val: String,
    multi_val: String,
}

fn main() {
    // Initialize the UI framework
    let ui = UI::init().unwrap();

    // Initialize the state of the application.
    //
    // The state singleton is wrapped in a `RefCell` to be able to dynamically
    // `.borrow()` a read-only reference at every event loop tick, as well
    // as `.borrow_mut()` a mutable reference every time a control is changed.
    //
    // Additionally, it is wrapped in a `Rc` so that all implicit references
    // captured by those closures point to the same underlying RefCell.
    //
    let state = Rc::new(RefCell::new(State {
        slider_val: 0,
        spinner_val: 0,
        entry_val: "".into(),
        multi_val: "".into(),
    }));

    // Create the layout and add the controls
    iui! { &ui,
        let contents = LayoutGrid(padded: true) {
            // This is position by slot (x,y); size (x,y); expansion, and
            // alignment (x,y). In this case: row 0, col 0, size = 1 by 1,
            // compress as much as possible, and align to the fill.
            (0,0) (1,1) Neither (Fill,Fill): let slider = Slider(1, 100)
            // This one is at column zero, row 1.
            (0,1) (1,1) Neither (Fill,Fill): let spinner = Spinbox(1, 100)
            // And so on...
            (0,3) (1,1) Neither (Fill,Fill): let _sp = HorizontalSeparator()
            (0,4) (1,1) Neither (Fill,Fill): let entry = Entry()
            // The multiline entry is at col 0, row 1, and expands vertically.
            (0,5) (1,1) Vertical (Fill,Fill): let multi = MultilineEntry()
            // Second column:
            (1,0) (1,1) Neither (Fill,Fill): let add_label = Label("")
            (1,1) (1,1) Neither (Fill,Fill): let sub_label = Label("")
            // We skip the slots 2 and 3 so that the text labels will align
            // with their inputs. This is important because the big text label
            // can expand vertically.
            (1,4) (1,1) Neither (Fill,Fill): let text_label = Label("")
            (1,5) (1,1) Neither (Fill,Fill): let bigtext_label = Label("")
        }
    }

    // Display the layout in a window
    let mut window =
        Window::new(&ui, "Input Output Test", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, contents);
    window.show(&ui);

    // These on_changed functions allow updating the application state when a
    // control changes its value.
    slider.on_changed(&ui, |val| {
        state.borrow_mut().slider_val = val;
    });
    spinner.on_changed(&ui, |val| {
        state.borrow_mut().spinner_val = val;
    });
    entry.on_changed(&ui, |val| {
        state.borrow_mut().entry_val = val;
    });
    multi.on_changed(&ui, |val| {
        state.borrow_mut().multi_val = val;
    });

    // Rather than just invoking ui.run(), using EventLoop gives a lot more
    // control over the user interface event loop.
    // The on_tick() callback is used to update the view against the state.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        move || {
            let state = state.borrow();

            // Update all the labels
            add_label.set_text(
                &ui,
                &format!("Added: {}", state.slider_val + state.spinner_val),
            );
            sub_label.set_text(
                &ui,
                &format!(
                    "Subtracted: {}",
                    state.slider_val - state.spinner_val
                ),
            );
            text_label.set_text(&ui, &format!("Text: {}", state.entry_val));
            bigtext_label
                .set_text(&ui, &format!("Multiline Text: {}", state.multi_val));
        }
    });
    event_loop.run(&ui);
}
