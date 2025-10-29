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
                            "{} \n, {:#?} {}, {}",
                            trade.symbol_name.clone(),
                            trade.trade_type,
                            trade.profit,
                            trade.time_open
                        ))
                    })
                    .collect::<Vec<Line>>();
                let mut line = Text::from("Symbol       Type    Profit   Time      Open");
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
                self.render_exposure(&mut line, &open_trades);
                line.extend(Text::from("-------------"));
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
struct Exposure {
    currency: String,
    symbol: String,
    profit: i32,
    sl: i32,
}
impl OpenTradesWidget {
    fn render_exposure(self, line: &mut Text, open_trades: &[OpenTrade]) {
        let mut sell_exposures = vec![];
        let mut buy_exposures = vec![];
        open_trades
            .iter()
            .map(|trade| {
                let (base_curr, quote_curr) = trade.symbol_name.split_at(3);
                match trade.trade_type {
                    mt5::OrderType::Buy => {
                        if !buy_exposures.contains(&quote_curr) {
                            buy_exposures.push(quote_curr);
                        }
                    }
                    mt5::OrderType::Sell => {
                        if !sell_exposures.contains(&base_curr) {
                            sell_exposures.push(base_curr);
                        }
                    }
                    _ => {}
                };
            })
            .for_each(drop);
        line.extend(Line::from("------- Exposure"));
        line.extend(Line::from("Buy Exposures"));
        line.extend(
            buy_exposures
                .iter()
                .map(|curr| Line::from(curr.to_string()))
                .collect::<Vec<Line>>(),
        );
        line.extend(Line::from("Sell Exposures"));
        line.extend(
            sell_exposures
                .iter()
                .map(|curr| Line::from(curr.to_string()))
                .collect::<Vec<Line>>(),
        );
    }
}
