// FIX: Can't have references to mt5?
use mt5::{self, OpenTrade, Order, OrderRequest, Symbol };
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
        let account = mt5::Account::get_account_info();

        //FIX: Source of mt5 bridge string?
        let bridge = "mt5";
        let symbols = Symbol::get_all(bridge);
        let title = Line::from("Calculator");

        //FIX: Source of Symbol String?
        let symbol = Symbol::get_symbol_data(bridge, "EURCAD");

        // FIX: Source of Timeframe, start and end date
        let hist_tick_data = symbol.get_historical_tick_data(bridge, "16408", 0, 15);

        let ticks = &hist_tick_data.ticks;

        let atr = (algo::calculate_atr(ticks) / symbol.point as f32 / 10.0) as u32;

        let order = Order::new_order(OrderRequest {
            account,
            // FIX: Can't have references to mt5?
            order_type: mt5::OrderType::OrderTypeSell,
            symbol: symbol.clone(),
            risk: 0.02,
            limit: Some(atr),
        })
        .execute_order(bridge);

        let symbol_text = Text::from_iter([
            symbol.name,
            symbol.sector,
            String::from(symbol.spread.to_string()),
        ]);
        Paragraph::new(symbol_text);
        let calculator_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        let calculator_text = Line::from("Some text");
        let x = format!("{:?}", hist_tick_data);

        let text = Text::from_iter([x]);
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
        // let inner_layout = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .constraints(layout_constraints)
        //     .split(area);

        CalculatorWidget::render_first_calculator(area, buf);
        // CalculatorWidget::render_second_calculator(inner_layout[1], buf);
    }
}
