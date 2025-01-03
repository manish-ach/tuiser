use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView, ListView, TextArea, Checkbox};
use cursive::Cursive;
use cursive::theme::{Palette, BorderStyle};
use cursive::menu;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let mut siv = cursive::default();
    // let theme = app_theme(&siv);
    // siv.set_theme(theme);

    siv.add_global_callback('q', Cursive::quit);
    menu(&mut siv);
    base(&mut siv);
    siv.run();
}

fn menu(siv: &mut Cursive) {
    let counter = AtomicUsize::new(1);
        // The menubar is a list of (label, menu tree) pairs.
        siv.menubar()
            // We add a new "File" tree
            .add_subtree(
                "File",
                menu::Tree::new()
                    .leaf("New", move |s| {
                        // Here we use the counter to add an entry
                        // in the list of "Recent" items.
                        let i = counter.fetch_add(1, Ordering::Relaxed);
                        let filename = format!("New {i}");
                        s.menubar()
                            .find_subtree("File")
                            .unwrap()
                            .find_subtree("Recent")
                            .unwrap()
                            .insert_leaf(0, filename, |_| ());

                        s.add_layer(Dialog::info("New file!"));
                    })
                    .subtree(
                        "Recent",
                        // The `.with()` method can help when running loops
                        // within builder patterns.
                        menu::Tree::new().with(|tree| {
                            for i in 1..5 {
                                tree.add_item(menu::Item::leaf(format!("Item {i}"), |_| ()).with(|s| {
                                    if i % 5 == 0 { s.disable(); }
                                }))
                            }
                        }),
                    )
                    // Delimiter are simple lines between items,
                    // and cannot be selected.
                    .delimiter()
                    .with(|tree| {
                        for i in 1..10 {
                            tree.add_leaf(format!("Option {i}"), |_| ());
                        }
                    }),
            )
            .add_subtree(
                "Help",
                menu::Tree::new()
                    .subtree(
                        "Help",
                        menu::Tree::new()
                            .leaf("General", |s| {
                                s.add_layer(Dialog::info("Help message!"))
                            })
                            .leaf("Online", |s| {
                                let text = "Google it yourself!\n\
                                            Kids, these days...";
                                s.add_layer(Dialog::info(text))
                            }),
                    )
                    .leaf("About", |s| {
                        s.add_layer(Dialog::info("Cursive v0.0.0"))
                    }),
            )
            .add_delimiter()
            .add_leaf("Quit", |s| s.quit());
        siv.set_autohide_menu(false);
}

//main workscreen, moduled for better reausability
fn base(siv: &mut Cursive) {
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::style::BaseColor::*;

            {
                //overriding some colors from the base palette.
                use cursive::style::Color::TerminalDefault;
                use cursive::style::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Cyan.light();
                palette[Highlight] = Cyan.dark();
                palette[HighlightInactive] = White.dark();
                palette[HighlightText] = Black.dark();
            }

            {
                // overriding some default styles.
                use cursive::style::Effect::*;
                use cursive::style::PaletteStyle::*;
                use cursive::style::Style;
                palette[Highlight] = Style::from(Cyan.light()).combine(Bold);
                palette[HighlightInactive] = Style::from(Black.dark()).combine(Bold);
                palette[EditableTextCursor] = Style::secondary().combine(Reverse).combine(Underline)
            }
        }),
    });
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((20, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("add", add_name))
        .child(Button::new("delete", delete_name))
        .child(Button::new("form", list))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView.fixed_width(15))
                .child(buttons),
        )
        .title("Select Profile")
        .fixed_size((50, 20)),
    );
}

//functions for buttons
fn list(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title("Please fill out this form")
            .button("Ok", |s| {
                s.pop_layer();
            })
            .content(
                ListView::new()
                    // Each child is a single-line view with a label
                    .child("Name", EditView::new().fixed_width(10))
                    .child(" ",DummyView.fixed_width(15))
                    .child("Presentation", TextArea::new().min_height(4))
                    .child(" ",DummyView.fixed_width(15))
                    .child(
                        "Receive spam?",
                        Checkbox::new().on_change(|s, checked| {
                            // Enable/Disable the next field depending on this checkbox
                            for name in &["email1", "email2"] {
                                s.call_on_name(name, |view: &mut EditView| {
                                    view.set_enabled(checked)
                                });
                                if checked {
                                    s.focus_name("email1").unwrap();
                                }
                            }
                        }),
                    )
                    .child(
                        "Email",
                        // Each child must have a height of 1 line,
                        // but we can still combine multiple views!
                        LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email1")
                                    .fixed_width(15),
                            )
                            .child(TextView::new("@"))
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email2")
                                    .fixed_width(10),
                            ),
                    )
                    .delimiter()
                    .child(
                        "Age",
                        // Popup-mode
                        SelectView::new()
                            .popup()
                            .item_str("0-18")
                            .item_str("19-30")
                            .item_str("31-40")
                            .item_str("41+"),
                    )
                    .with(|list| {
                        // child editviews
                        for i in 0..10 {
                            list.add_child(
                                &format!("demo {i}"),
                                EditView::new(),
                            );
                        }
                    })
                    .scrollable(),
            ),
    );
}
fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name")
                .fixed_width(10),
        )
        .title("Enter new username")
        .button("Ok", |s| {
            let name = s
                .call_on_name("name", |v: &mut EditView| v.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}
fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No record to remove")),
        Some(value) => {
            select.remove_item(value);
        }
    }
}
fn on_submit(s: &mut Cursive, name: &str) {
    s.add_layer(
        Dialog::text(format!("Name: {}\ndemo run", name))
            .title(format!("{}'s information", name))
            .button("quit", |s| {
                s.pop_layer();
            }),
    );
}
