#[allow(unused)]
#[allow(dead_code)]
use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction},
    prelude::{Buffer, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
mod calculator;
mod chart;
mod counter;
mod currencies;

use calculator::CalculatorWidget;
use chart::ChartWidget;
use counter::CounterWidget;
use currencies::CurrencyWidget;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        // terminal.show_cursor()?;

        Ok(())
    }

    fn app_page(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('C') => {}
            _ => todo!(),
        }
    }

    fn draw(&self, frame: &mut Frame) {
        // Set constraints and Layout
        let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.clone())
            .split(frame.area());

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(layout[0]);

        let mut rects: Vec<Rect> = vec![Rect::default(); 5];
        rects.insert(0, inner_layout[0]);
        rects.insert(1, inner_layout[1]);
        rects.insert(2, layout[1]);

        // Prep the Widgets to be built

        //Render Widgets
        frame.render_widget(CurrencyWidget::default(), rects[0]);
        frame.render_widget(CalculatorWidget::default(), rects[1]);
        frame.render_widget(ChartWidget::default(), rects[2]);
    }
    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char('C') => self.app_page(key_event.code),
            // KeyCode::Char('j') => frame.kkkkk
            KeyCode::Null => {}
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
    fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

mod tests {}
