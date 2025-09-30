use mt5::Symbol;
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Line, Rect, Text},
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub struct CurrencyWidget {}

impl CurrencyWidget {
    fn set_layout() {}
}
impl Default for CurrencyWidget {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for CurrencyWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {

        let symbols = Symbol::get_all("mt5");

        let currencies_block = Block::bordered()
            .title("Currencies")
            .title_top("sfed")
            .title_alignment(Alignment::Center)
            .border_set(border::DOUBLE);
        let currency_text = symbols
            .iter()
            .map(|symbol| Line::from(format!("{:#?}", symbol.name)).alignment(Alignment::Right))
            .collect::<Vec<Line>>();

        let mut text = Text::from_iter([Line::from("Currencies").alignment(Alignment::Center)]);
        text.extend(currency_text);
        Paragraph::new(text)
            .centered()
            .block(currencies_block)
            .render(area, buf);
    }
}
