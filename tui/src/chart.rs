use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Line, Rect, Text},
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub struct ChartWidget;

impl ChartWidget {}
impl Default for ChartWidget {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for ChartWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Chart Region");

        let currencies_block = Block::bordered()
            .title("Charts")
            .title_top("sfed")
            .border_set(border::DOUBLE);

        let chart_content = Line::from("Content");

        let chart_text = Text::from_iter([title, chart_content]);

        Paragraph::new(chart_text)
            .block(currencies_block)
            .render(area, buf);
    }
}
