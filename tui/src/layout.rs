use crate::widgets::{
    account::AccountWidget, calculator::CalculatorWidget, chart::ChartWidget, news::NewsWidget,
    opentrades::OpenTradesWidget, risk::RiskWidget,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn create_layout(frame: &mut Frame) {
    // Set constraints and Layout
    // let layout = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints(constraints.clone())
    //     .split(frame.area());

    // Account Details && Horizontal Rest
    let constraints = vec![Constraint::Percentage(10), Constraint::Percentage(90)];
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());
    let account_details_area = layout[0];

    // Open Trades & Vertical Rest
    let constraints = vec![Constraint::Percentage(30), Constraint::Percentage(70)];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(layout[1]);
    let open_trades_area = layout[0];

    // Chart Area, News & Horizontal Rest
    let constraints = vec![Constraint::Percentage(70), Constraint::Percentage(30)];
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(layout[1]);
    let chart_news_area = layout[0];
    let risk_calculator_area = layout[1];

    // Chart & News
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(chart_news_area);
    let chart_area = layout[0];
    let news_area = layout[1];

    // Risk && Calculator
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(risk_calculator_area);
    let risk_area = layout[0];
    let calculator_area = layout[1];

    let mut rects: Vec<Rect> = vec![Rect::default(); 5];
    // rects.insert(0, account_details_area);
    // rects.insert(1, inner_layout[1]);
    // rects.insert(2, layout[1]);

    // Prep the Widgets to be built

    //Render Widgets
    frame.render_widget(AccountWidget, account_details_area);
    frame.render_widget(OpenTradesWidget, open_trades_area);
    frame.render_widget(ChartWidget, chart_area);
    frame.render_widget(NewsWidget, news_area);
    frame.render_widget(RiskWidget, risk_area);
    frame.render_widget(CalculatorWidget, calculator_area);
}
