use ratatui::{
    layout::Alignment,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

pub struct RiskWidget;
impl Default for RiskWidget {
    fn default() -> Self {
        Self {}
    }
}
impl Widget for RiskWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = "Risk Analysis";
        let risk_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        Paragraph::new(Text::from("xx"))
            .block(risk_block)
            .render(area, buf);
    }
}
