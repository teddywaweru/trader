// FIX: Can't have references to mt5?
use mt5::{self, OpenTrade, Order, OrderRequest, Symbol};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Line, Rect, Span, Text},
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
        let symbol_name = "AUDCHF";
        let risk = 0.01;
        let title = Line::from("Calculator");
        // Data and Logic
        let account = mt5::Account::get_account_info();

        let open_trades = OpenTrade::get_all("mt5");

        // UI
        let calculator_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        let text = Text::from_iter([
            format!("Trading: {symbol_name}"),
            format!("Risk: {}", 0.001),
        ]);
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
    pub fn execute(symbol_name: &str) {
        //FIX: Source of mt5 bridge string?
        // FIX: Should not be loading this data twice... both in render and execution
        let account = mt5::Account::get_account_info();
        let bridge = "mt5";
        let symbols = Symbol::get_all(bridge);

        //FIX: Source of Symbol String?
        let symbol = Symbol::get_symbol_data(bridge, symbol_name);

        // FIX: Source of Timeframe, start and end date
        let hist_tick_data = symbol.get_historical_tick_data(bridge, "16408", 0, 15);

        let ticks = &hist_tick_data.ticks;

        let atr = (algo::calculate_atr(ticks) / symbol.point / 10.0) as u32;

        let orders = vec![
            Order::new_order(OrderRequest {
                account: account.clone(),
                // FIX: Can't have references to mt5?
                order_type: mt5::OrderType::Buy,
                symbol: symbol.clone(),
                risk: 0.001,
                limit: Some(atr),
            }),
            Order::new_order(OrderRequest {
                account,
                // FIX: Can't have references to mt5?
                order_type: mt5::OrderType::Buy,
                symbol: symbol.clone(),
                risk: 0.001,
                limit: Some(atr),
            }),
        ];

        orders
            .iter()
            .map(|order| order.execute_order(bridge))
            .for_each(drop);
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
