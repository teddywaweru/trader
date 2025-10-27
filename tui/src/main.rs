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
mod account;
mod risk;
mod news;
mod opentrades;

use calculator::CalculatorWidget;
use chart::ChartWidget;
use counter::CounterWidget;
use currencies::CurrencyWidget;
use account::AccountWidget;
use risk::RiskWidget;
use news::NewsWidget;
use opentrades::OpenTradesWidget;

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
        // let layout = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(constraints.clone())
        //     .split(frame.area());

        // Account Details && Horizontal Rest
        let constraints = vec![Constraint::Percentage(10), Constraint::Percentage(90)];
        let layout = Layout::default().direction(Direction::Vertical).constraints(constraints).split(frame.area());
        let account_details_area = layout[0];

        // Open Trades & Vertical Rest
        let constraints = vec![Constraint::Percentage(30), Constraint::Percentage(70)];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(layout[1]);
        let open_trades_area = layout[0];

        // Chart Area, News & Horizontal Rest
        let constraints = vec![Constraint::Percentage(70), Constraint::Percentage(30)];
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(layout[1]);
        let chart_news_area = layout[0];
        let risk_calculator_area = layout[1];

        // Chart & News
        let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(chart_news_area);
        let chart_area = layout[0];
        let news_area = layout[1];

        // Risk && Calculator
        let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(risk_calculator_area);
        let risk_area = layout[0];
        let calculator_area = layout[1];

        let mut rects: Vec<Rect> = vec![Rect::default(); 5];
        // rects.insert(0, account_details_area);
        // rects.insert(1, inner_layout[1]);
        // rects.insert(2, layout[1]);

        // Prep the Widgets to be built

        //Render Widgets
        frame.render_widget(AccountWidget::default(), account_details_area);
        frame.render_widget(OpenTradesWidget::default(), open_trades_area);
        frame.render_widget(ChartWidget::default(), chart_area);
        frame.render_widget(NewsWidget::default(), news_area);
        frame.render_widget(RiskWidget::default(), risk_area);
        frame.render_widget(CalculatorWidget::default(), calculator_area);
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
            // KeyCode::Char('r') => CalculatorWidget::
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
