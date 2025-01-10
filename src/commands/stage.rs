use crate::library::routine::Routine;

pub(crate) fn stage(routine_path: &String, message: &Option<String>) {
    Routine::read(routine_path)
        .unwrap_or_else(|error| {
            panic!("Error reading routine: {}", error);
        })
        .stage(message)
        .expect("Error staging routine");
}
