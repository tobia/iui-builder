/// Parse a tree of IUI Controls
#[macro_export]
macro_rules! iui {

    // ---------------------- Controls without children -----------------------

    // Button
    [ $ui:expr ,
        let $ctl:ident = Button ( $text:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Button::new($ui, $text);
    ];

    // Entry
    [ $ui:expr ,
        let $ctl:ident = Entry ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Entry::new($ui);
    ];

    // Checkbox
    [ $ui:expr ,
        let $ctl:ident = Checkbox ( $text:expr $( , checked: $checked:expr )? )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Checkbox::new($ui, $text);
        $( $ctl.set_checked($ui, $checked); )?
    ];

    // Combobox
    [ $ui:expr ,
        let $ctl:ident = Combobox ( $( selected: $selected:expr )? )
        { $( $option:expr ),* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Combobox::new($ui);
        $( $ctl.append($ui, $option); )*
        $( $ctl.set_selected($ui, $selected); )?
    ];

    // HorizontalSeparator
    [ $ui:expr ,
        let $ctl:ident = HorizontalSeparator ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::HorizontalSeparator::new($ui);
    ];

    // Label
    [ $ui:expr ,
        let $ctl:ident = Label ( $text:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Label::new($ui, $text);
    ];

    // MultilineEntry
    [ $ui:expr ,
        let $ctl:ident = MultilineEntry ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::MultilineEntry::new($ui);
    ];

    // Slider
    [ $ui:expr ,
        let $ctl:ident = Slider ( $min:expr , $max:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Slider::new($ui, $min, $max);
    ];

    // Spacer
    [ $ui:expr ,
        let $ctl:ident = Spacer ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Spacer::new($ui);
    ];

    // Spinbox, limited
    [ $ui:expr ,
        let $ctl:ident = Spinbox ( $min:expr , $max:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Spinbox::new($ui, $min, $max);
    ];

    // Spinbox, unlimited
    [ $ui:expr ,
        let $ctl:ident = Spinbox ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Spinbox::new_unlimited($ui);
    ];

    // ----------------- Controls with children (Containers) ------------------

    // Group
    [ $ui:expr ,
        let $ctl:ident = Group ( $title:expr $( , margined: $margined:expr )? )
        { $(
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )? }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::Group::new($ui, $title);
        $( $ctl.set_margined($ui, $margined); )?
        $(
            iui! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.set_child($ui, $child.clone());
        )?
    ];

    // HorizontalBox
    [ $ui:expr ,
        let $ctl:ident = HorizontalBox ( $( padded: $padded:expr )? )
        { $(
            $strategy:ident :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::HorizontalBox::new($ui);
        $( $ctl.set_padded($ui, $padded); )?
        $(
            iui! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($ui, $child.clone(),
                        iui::controls::LayoutStrategy::$strategy);
        )*
    ];

    // LayoutGrid
    [ $ui:expr ,
        let $ctl:ident = LayoutGrid ( $( padded: $padded:expr )? )
        { $(
            ( $x:expr , $y:expr ) ( $xspan:expr , $yspan:expr )
            $expand:ident ( $halign:ident , $valign:ident ) :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::LayoutGrid::new($ui);
        $( $ctl.set_padded($ui, $padded); )?
        $(
            iui! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($ui, $child.clone(), $x, $y, $xspan, $yspan,
                        iui::controls::GridExpand::$expand,
                        iui::controls::GridAlignment::$halign,
                        iui::controls::GridAlignment::$valign);
        )*
    ];

    // TabGroup
    [ $ui:expr ,
        let $ctl:ident = TabGroup ()
        { $(
            ( $name:expr $( , margined: $margined:expr )? ) :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::TabGroup::new($ui);
        $(
            iui! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            let __tab_n = $ctl.append($ui, $name, $child.clone());
            $( $ctl.set_margined($ui, __tab_n - 1, $margined); )?
        )*
    ];

    // VerticalBox
    [ $ui:expr ,
        let $ctl:ident = VerticalBox ( $( padded: $padded:expr )? )
        { $(
            $strategy:ident :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = iui::controls::VerticalBox::new($ui);
        $( $ctl.set_padded($ui, $padded); )?
        $(
            iui! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($ui, $child.clone(),
                        iui::controls::LayoutStrategy::$strategy);
        )*
    ];
}
