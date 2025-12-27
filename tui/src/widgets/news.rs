use ratatui::{layout::Alignment, text::Text, widgets::{Block, Paragraph, Widget } };
pub struct NewsWidget;
impl Default for NewsWidget{
    fn default() -> Self {
        Self {  }
    }
}
impl Widget for NewsWidget{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let title = "News";
        let risk_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        Paragraph::new(Text::from("xx"))
            .block(risk_block)
            .render(area, buf);
    }
}

