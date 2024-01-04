#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Model {
    pub counter: i32,
    pub running_state: RunningState,
    pub tab: usize,
}
