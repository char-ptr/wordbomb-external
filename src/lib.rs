pub mod cv;
pub mod faker_input;
pub enum Communication {
    Ignore,
    WordPart(String),
    GameStart((i32, i32)),
}
