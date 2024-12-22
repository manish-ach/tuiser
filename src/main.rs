use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;

fn main() {
    let mut tui = cursive::default();
    tui.add_global_callback('q', Cursive::quit);

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("add", add_name))
        .child(Button::new("delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    tui.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView.fixed_size((25, 10)))
                .child(buttons),
        )
        .title("Select Profile")
        .fixed_size((50, 20)),
    );
    tui.run();
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
