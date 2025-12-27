use mt5::OpenTrade;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    widgets::{Block, Widget},
};
pub struct TotalsWidget;

impl Default for TotalsWidget {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for TotalsWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let open_trades = OpenTrade::get_all("mt5");
        let mut totals_text = Span::from("");
        match open_trades {
            Ok(open_trades) => {
                let (totals, swap) = open_trades
                    .iter()
                    .map(|trade| (trade.profit, trade.swap))
                    .collect::<Vec<(f32, f32)>>()
                    .iter()
                    .copied()
                    .unzip::<f32, f32, Vec<f32>, Vec<f32>>();
                totals_text = Span::styled(
                    format!(
                        "Total:{:#?} Swap: {:#?}",
                        totals.iter()
                        .sum::<f32>(), swap.iter().sum::<f32>()
                    ),
                    Style::default().fg(Color::Yellow),
                );
            }
            Err(e) => {
                todo!()
            }
        }
        Paragraph::new(totals_text)
            .block(Block::bordered())
            .render(area, buf);
    }
}
