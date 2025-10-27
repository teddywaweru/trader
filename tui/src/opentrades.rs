use mt5::OpenTrade;
use ratatui::{
    layout::Alignment,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct OpenTradesWidget;
impl Default for OpenTradesWidget {
    fn default() -> Self {
        Self {}
    }
}
impl Widget for OpenTradesWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let open_trades = OpenTrade::get_all("mt5");

        // UI
        let title = "Open Trades";
        let calculator_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        match open_trades {
            Ok(open_trades) => {
                let mut open_trades_text = open_trades
                    .iter()
                    .map(|trade| {
                        Line::from(format!(
                            "{} \n, {}, {}",
                            trade.symbol_name.clone(),
                            trade.profit,
                            trade.time_open
                        ))
                    })
                    .collect::<Vec<Line>>();
                let mut line = Text::from("Symbol    Profit   Time Open");
                line.extend(open_trades_text);
                line.extend(Text::from("-------------"));
                line.extend(Text::from(format!(
                    "Total:{}",
                    open_trades
                        .iter()
                        .map(|trade| trade.profit + trade.swap)
                        .collect::<Vec<f32>>()
                        .iter()
                        .sum::<f32>()
                )));
                Paragraph::new(line)
                    .block(calculator_block)
                    .render(area, buf);
            }
            Err(r) => {
                Paragraph::new(Text::from_iter([
                    // symbol.name,
                    // symbol.sector,
                    // String::from(symbol.spread.to_string()),
                    format!("Error:{}", r),
                ]))
                .block(calculator_block)
                .render(area, buf);
            }
        }
    }
}
