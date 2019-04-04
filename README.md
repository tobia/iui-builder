# Macro-based builder for IUI

This package contains a single macro `iui!` that allows building [IUI](https://github.com/rust-native-ui/libui-rs/tree/0.3.0) user interfaces using a hierarchical, declarative DSL. The current version of the macro targets IUI stable 0.3.0.

For example, this is the `iui!`-defined version of the standard `inputs.rs` example:

```rust
// Create the layout and add the controls
iui! { &ui,
    let contents = HorizontalBox() {
        Stretchy: let input_group = Group("Inputs") {
            let input_vbox = VerticalBox(padded: true) {
                Compact: let slider = Slider(1, 100)
                Compact: let spinner = Spinbox(1, 100)
                Compact: let _sp = Spacer()
                Compact: let _sp = HorizontalSeparator()
                Compact: let _sp = Spacer()
                Compact: let entry = Entry()
                Stretchy: let multi = MultilineEntry()
            }
        }
        Stretchy: let output_group = Group("Outputs") {
            let output_vbox = VerticalBox() {
                Compact: let add_label = Label("")
                Compact: let sub_label = Label("")
                Compact: let text_label = Label("")
                Stretchy: let bigtext_label = Label("")
            }
        }
    }
}

// Add the layout to the window
window.set_child(&ui, contents);

// Update the application state when a control changes its value.
slider.on_changed(&ui, |val| {
    state.borrow_mut().slider_val = val;
});
```

The preceding macro is expanded into the following code (with namespaces removed and newlines added for clarity; in the generated code, all IUI type names are generated with full namespaces):

```rust
#[allow(unused_mut)]
let mut contents = HorizontalBox::new(&ui);

#[allow(unused_mut)]
let mut input_group = Group::new(&ui, "Inputs");

#[allow(unused_mut)]
let mut input_vbox = VerticalBox::new(&ui);
input_vbox.set_padded(&ui, true);

#[allow(unused_mut)]
let mut slider = Slider::new(&ui, 1, 100);
input_vbox.append(&ui, slider.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut spinner = Spinbox::new(&ui, 1, 100);
input_vbox.append(&ui, spinner.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut _sp = Spacer::new(&ui);
input_vbox.append(&ui, _sp.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut _sp = HorizontalSeparator::new(&ui);
input_vbox.append(&ui, _sp.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut _sp = Spacer::new(&ui);
input_vbox.append(&ui, _sp.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut entry = Entry::new(&ui);
input_vbox.append(&ui, entry.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut multi = MultilineEntry::new(&ui);
input_vbox.append(&ui, multi.clone(), LayoutStrategy::Stretchy);

input_group.set_child(&ui, input_vbox.clone());
contents.append(&ui, input_group.clone(), LayoutStrategy::Stretchy);

#[allow(unused_mut)]
let mut output_group = Group::new(&ui, "Outputs");

#[allow(unused_mut)]
let mut output_vbox = VerticalBox::new(&ui);

#[allow(unused_mut)]
let mut add_label = Label::new(&ui, "");
output_vbox.append(&ui, add_label.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut sub_label = Label::new(&ui, "");
output_vbox.append(&ui, sub_label.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut text_label = Label::new(&ui, "");
output_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);

#[allow(unused_mut)]
let mut bigtext_label = Label::new(&ui, "");
output_vbox.append(&ui, bigtext_label.clone(), LayoutStrategy::Stretchy);

output_group.set_child(&ui, output_vbox.clone());
contents.append(&ui, output_group.clone(), LayoutStrategy::Stretchy);
```

## Syntax

An explanation of the macro syntax follows. You may additionally look at the [examples](examples) for practical usage of all the controls, or at the macro [source code](src/lib.rs) for the specific syntax rules.

- The first argument to the macro should be a reference to the IUI Context, typically called `&ui`, followed by a comma.
- The macro body should contain a ***single*** root control definition, which looks like a variable definition *without a trailing semicolon.*
- Controls that are "containers", such as Boxes and Groups, may contain nested definitions inside curly braces `{ }`. The nesting is only used by the macro to build the control graph. In the generated source code, the variables are all defined in the same block as the `iui!` macro.
- Each control definition starts with a single `let` keyword, but is expanded into a `let mut` variable with unused warning suppression `#[allow(unused_mut)]` to allow setting properties or adding children, if requested in the macro syntax.
- Each control *requires a variable name,* because several features of the `iui!` macro depend on being able to reference the control. If you don't need to reference the control outside of the `iui!` macro, you can use a leading underscore as a reminder that the variable is unused (by you) and/or reuse variable names, *paying attention to the next point.*
- Each control must have a ***unique*** variable name with respect to its ancestors and descendents. Its siblings, "uncles", and "cousins" may reuse the same name.
- In each `let`-like variable definition, the control type name ***must*** be followed by a single set of parentheses `( )` and ***may*** be followed by a single set of braces `{ }`. The contents of the parentheses and braces, and whether the braces are required, depend on the particular control being created.
- As a guiding principle, any required parameters in the control constructor (`new()` function) are accepted as "positional arguments" in the parentheses; any optional properties (`set_*()` methods) are accepted as "named arguments" in the parentheses; and any children can be defined inside the braces.
- Most containers require additional arguments for each child that is added, such as `LayoutStrategy` for boxes. These are accepted ***before*** the child control definition and are followed by a single colon `:`.
