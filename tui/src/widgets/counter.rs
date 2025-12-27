use ratatui::{
    prelude::{Buffer, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
pub struct CounterWidget {
    counter: u8,
}

impl Default for CounterWidget {
    fn default() -> Self {
        Self {
            counter: Default::default(),
        }
    }
}
impl Widget for CounterWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        //Generate Rects here?
        //Call the Widgets with their individual areas?
        let title = Line::from("Welcome");

        let instructions = Line::from(vec![
            "Decrement".into(),
            "<Left>".into(),
            "Increment".into(),
            "<Right>".into(),
            "Quit".into(),
            "<Q>".into(),
        ]);

        let intro = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value:".into(),
            self.counter.to_string().yellow(),
            format!("Width:{}, Height:{}", area.width, area.height).into(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(intro)
            .render(area, buf);
    }
}
