use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Line, Rect, Text},
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};
pub struct AccountWidget;
impl Default for AccountWidget {
    fn default() -> Self {
        Self {}
    }
}
impl AccountWidget {}
impl Widget for AccountWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let account = mt5::Account::get_account_info();
        // let symbols = Symbol::get_all("mt5");
        let currencies_block = Block::bordered()
            .title("Account Details")
            .title_top(format!("{}: {}: {}", account.name, account.account_number, account.currency))
            .title_alignment(Alignment::Center)
            .border_set(border::DOUBLE);
        // let currency_text = symbols
        //     .iter()
        //     .map(|symbol| Line::from(format!("{:#?}", symbol.name)).alignment(Alignment::Right))
        //     .collect::<Vec<Line>>();

        let line = Text::from_iter([
            Line::from(format!("Current Balance: {}", account.current_profit)),
            Line::from(format!("Current Equity: {}", account.current_equity)),
            Line::from(format!("Current Profit: {}", account.current_profit)),
            Line::from(format!("Leverage: {}", account.leverage)),
            Line::from(format!("Free Margin: {}", account.free_margin)),
            Line::from(format!("Current Time: {}", account.current_time)),
        ]);

        Paragraph::new(line)
            .centered()
            .block(currencies_block)
            .render(area, buf);
    }
}
