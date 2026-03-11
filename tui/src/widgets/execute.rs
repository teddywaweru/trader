use ratatui::widgets::{Block, Paragraph, Widget};
pub struct ExecuteWidget;
impl Default for ExecuteWidget {
    fn default() -> Self {
        Self {}
    }
}
impl Widget for ExecuteWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("sdfwe")
            .block(Block::bordered().title("Execution"))
            .render(area, buf);
    }
}
