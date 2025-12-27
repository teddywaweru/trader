use crate::{ App, widgets::calculator::CalculatorWidget };
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
pub fn handle_key_event(app: &mut App, key_event: KeyEvent) -> () {
    match key_event.code {
        KeyCode::Char('q') => exit(app),
        KeyCode::Left => execute(app),
        // KeyCode::Right => increment_counter(),
        // KeyCode::Char('C') => app_page(key_event.code),
        KeyCode::Char('r') => {}
        KeyCode::Char('j') => {}
        KeyCode::Char('t') => {
            execute(app);
        }
        KeyCode::Null => {}
        _ => {}
    }
}
fn exit(app: &mut App) {
    app.exit = true;
}
fn execute(app: &mut App) {
    CalculatorWidget::execute("AUDCHF");
}
