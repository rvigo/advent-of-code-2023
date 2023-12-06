pub mod answer;
pub mod solution;

#[macro_export]
macro_rules! load_input {
    () => {
        std::fs::read_to_string("input.txt").unwrap()
    };
}
