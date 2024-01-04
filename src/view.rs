use crate::model::Model;
use ratatui::{prelude::*, widgets::*};

pub fn view(model: &mut Model, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("Counter: {}", model.counter)),
        f.size(),
    );
}
