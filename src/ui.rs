use cursive::Cursive;
use cursive::views::{Dialog, TextView};

pub fn cursive_main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Starts the event loop.
    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Did you do the thing?")
        .title("Question 1")
        .button("Yes!", |s| show_answer(s, "Great job!"))
        .button("No", |s| show_answer(s, "What the hell man.")));
}
fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit()));
}
