use mt5::OpenTrade;
use ratatui::{
    layout::{Alignment, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct OpenTradesWidget {
    open_trades: Vec<OpenTrade>,
}
impl Default for OpenTradesWidget {
    fn default() -> Self {
        Self {
            open_trades: vec![],
        }
    }
}
impl OpenTradesWidget {
    pub fn new(open_trades: Result<Vec<OpenTrade>, Box<dyn std::error::Error>>) -> Self {
        match open_trades{
            Ok(open_trades) => {
                OpenTradesWidget {
                    open_trades,
                }
            }
            Err(e) =>
            {
                OpenTradesWidget {
                    open_trades: vec![OpenTrade::default()]
                }
            }
        }
    }
}
impl Widget for OpenTradesWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // UI
        let title = "Open Trades";
        let calculator_block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);

        Paragraph::new("")
            .block(calculator_block.clone())
            .render(area, buf);

        let area = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Max(8),
                ratatui::layout::Constraint::Max(6),
                ratatui::layout::Constraint::Min(0),
            ])
            .split(area);
        let open_trades = OpenTrade::get_all("mt5");
        match open_trades {
            Ok(open_trades) => {
                let mut trade_symbol_name_text: Text = Text::from("Symbol");
                let mut trade_type_text: Text = Text::from("Type");
                let mut trade_profit_text: Text = Text::from("Profit");
                let mut open_trades_text: Text =
                    Text::from("Symbol       Type    Profit   Time      Open");
                open_trades
                    .iter()
                    .map(|trade| {
                        trade_symbol_name_text.extend(Line::from(Span::styled(
                            format!("{}", trade.symbol_name),
                            Style::default().fg(Color::White),
                        )));
                        trade_type_text.extend(Line::from(Span::styled(
                            format!("{:#?}", trade.trade_type),
                            match trade.trade_type {
                                mt5::OrderType::Buy => Style::default().fg(Color::Blue),
                                mt5::OrderType::Sell => Style::default().fg(Color::Yellow),
                                _ => todo!(),
                            },
                        )));
                        trade_profit_text.extend(Line::from(Span::styled(
                            format!("{:#?}", trade.profit),
                            match trade.profit {
                                ..0.0 => Style::default().fg(Color::Red),
                                0.0 => Style::default().fg(Color::White),
                                1.0.. => Style::default().fg(Color::Green),
                                _ => todo!(),
                            },
                        )));
                    })
                    .for_each(drop);
                open_trades_text.extend(
                    vec![
                        Span::styled(
                            format!(
                                "Total:{}",
                                open_trades
                                    .iter()
                                    .map(|trade| trade.profit + trade.swap)
                                    .collect::<Vec<f32>>()
                                    .iter()
                                    .sum::<f32>()
                            ),
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::from("-------------"),
                        Span::styled(
                            format!(
                                "Total:{}",
                                open_trades
                                    .iter()
                                    .map(|trade| trade.profit + trade.swap)
                                    .collect::<Vec<f32>>()
                                    .iter()
                                    .sum::<f32>()
                            ),
                            Style::default().fg(Color::Yellow),
                        ),
                    ], // .iter()
                );
                Paragraph::new(trade_symbol_name_text).render(area[0], buf);
                Paragraph::new(trade_type_text).render(area[1], buf);
                Paragraph::new(trade_profit_text).render(area[2], buf);
            }
            Err(r) => {
                Paragraph::new(Text::from_iter([
                    // symbol.name,
                    // symbol.sector,
                    // String::from(symbol.spread.to_string()),
                    format!("Error:{r}"),
                ]))
                .block(calculator_block)
                .render(area[0], buf);
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
