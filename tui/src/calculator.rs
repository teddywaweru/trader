use mt5::{self, Symbol, Symbols};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Line, Rect, Text},
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub struct CalculatorWidget;
impl Default for CalculatorWidget {
    fn default() -> Self {
        Self {}
    }
}
impl CalculatorWidget {
    fn render_first_calculator(area: Rect, buf: &mut Buffer) {
        // let account = mt5::Account::get_account_info();

        //FIX: Source of mt5 string?
        let bridge = "mt5";
        let symbols = Symbols::get_symbols(bridge);
        let title = Line::from("Calculator");

        //FIX: Source of Symbol String?
        let symbol = Symbol::get_symbol_data(bridge, "EURUSD");

        let symbol_text =
            Text::from_iter([symbol.name, symbol.sector, String::from(symbol.spread.to_string())]);
        Paragraph::new(symbol_text);
        let calculator_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        let calculator_text = Line::from("Some text");

        let text = Text::from_iter([calculator_text]);
        Paragraph::new(text)
            .block(calculator_block)
            .render(area, buf);
    }
    fn render_second_calculator(area: Rect, buf: &mut Buffer) {
        let title = Line::from("Analysis");

        let calculator_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Left);

        let calculator_text = Line::from("Some text");

        let text = Text::from_iter([calculator_text]);
        Paragraph::new(text)
            .block(calculator_block)
            .render(area, buf);

        //TODO: Get the value selected in the Currencies Tab?
    }
}

impl Widget for CalculatorWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout_constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(layout_constraints)
            .split(area);

        CalculatorWidget::render_first_calculator(inner_layout[0], buf);
        CalculatorWidget::render_second_calculator(inner_layout[1], buf);
    }
}
