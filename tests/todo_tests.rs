use anyhow::Result;
use eval_bot::todo::{self, TodoList};
use pretty_assertions::assert_eq;
use serenity::{model::user::User, utils::CustomMessage};

static USER_NAME: &str = "randomPoison";

/// Builds a [Message] from the given `text`.
fn send_message(state: &mut TodoList, text: &str) -> Result<String> {
    let mut builder = CustomMessage::new();
    builder.content(text.to_string());

    let mut user = User::default();
    user.name = USER_NAME.into();
    builder.author(user);

    let message = builder.build();

    todo::handle_message(state, &message)
}

// Adds an item and verifies that the response is correct.
fn add_item(state: &mut TodoList, key: &str, priority: u32) {
    let response = send_message(state, &format!("!todo {key}")).unwrap();

    let expected = match priority {
        1 => format!("Added item {key:?} to your list"),
        _ => format!("Updated item {key:?}, priority is {priority}"),
    };
    assert_eq!(expected, response);
}

/// Tests that an item can be added from the list, displayed, and then removed.
#[test]
fn add_display_remove() {
    let mut state = TodoList::default();

    // Add an item with the key "foo" to the list.
    add_item(&mut state, "foo", 1);

    // Verify that the item can be displayed in the TODO list.
    let response = send_message(&mut state, "!todo").unwrap();
    assert_eq!(
        format!(
            "TODO list for {USER_NAME}:\n\
            > [ ] foo\n"
        ),
        response,
    );

    // Remove the item from the list.
    let response = send_message(&mut state, "!todo rm foo").unwrap();
    assert_eq!(r#"Removed "foo" from your list"#, response);

    // Verify that the list is now empty when printed.
    let response = send_message(&mut state, "!todo").unwrap();
    assert_eq!(format!("TODO list for {USER_NAME}:\n"), response);
}

// Verifies that items in the TODO list are displayed in priority order.
#[test]
fn priority_sort() {
    let mut state = TodoList::default();

    // Create 3 TODO items, each with different priority values.
    add_item(&mut state, "foo", 1);
    add_item(&mut state, "foo", 2);
    add_item(&mut state, "foo", 3);

    add_item(&mut state, "foo bar", 1);
    add_item(&mut state, "foo bar", 2);

    add_item(&mut state, "foo bar baz", 1);

    // Verify that the items are displayed in the correct order.
    let response = send_message(&mut state, "!todo").unwrap();
    assert_eq!(
        format!(
            "TODO list for {USER_NAME}:\n\
            > [ ] foo\n\
            > [ ] foo bar\n\
            > [ ] foo bar baz\n"
        ),
        response,
    );
}

/// Verifies that items can be marked done.
#[test]
fn mark_items_done() {
    let mut state = TodoList::default();

    // Create 2 TODO items with different priority values so that they'll print
    // in a deterministic order.
    add_item(&mut state, "foo", 1);
    add_item(&mut state, "foo", 2);

    add_item(&mut state, "foo bar", 1);

    let response = send_message(&mut state, "!todo done foo").unwrap();
    assert_eq!(r#"Marked "foo" as done"#, response);

    // Verify that the items are displayed in the correct order.
    let response = send_message(&mut state, "!todo").unwrap();
    assert_eq!(
        format!(
            "TODO list for {USER_NAME}:\n\
            > [X] foo\n\
            > [ ] foo bar\n"
        ),
        response,
    );
}
