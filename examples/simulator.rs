use amm::{cpmm::*, pidmm::*, LiquidityPool};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::ThreadRng, Rng};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    Frame, Terminal,
};

struct App {
    data1: Vec<(f64, f64)>,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
    // amm: PIDMarketMaker,
    amm: ConstantProductMarketMaker,
    x: f64,
    rng: ThreadRng,
}

impl App {
    fn new() -> App {
        // let mut amm = PIDMarketMaker::default();
        // amm.target_price = dec!(2);
        // amm.pid.kp = dec!(0.1);
        // amm.pid.ki = dec!(0.0);
        // amm.pid.kd = dec!(0.0);
        
        let mut amm = ConstantProductMarketMaker::default();
        
        amm.add_liquidity(dec!(1000), dec!(1200));

        let mut data1 = vec![];
        let mut data2 = vec![];

        let mut rng = rand::thread_rng();

        for x in 0..200 {
            let buy_a = Decimal::from(rng.gen_range(1..=3));
            let buy_b = Decimal::from(rng.gen_range(1..=3));

            amm.swap_a(buy_a);
            amm.swap_b(buy_b);

            let price_a = amm.price_a(dec!(1)).round_dp(2);
            let price_b = amm.price_b(dec!(1)).round_dp(2);

            data1.push((x as f64 * 0.1, price_a.to_f64().unwrap()));
            data2.push((x as f64 * 0.1, price_b.to_f64().unwrap()));
        }

        App {
            data1,
            data2,
            window: [0.0, 20.0],
            amm,
            x: 20.0,
            rng,
        }
    }

    fn on_tick(&mut self) {
        for _ in 0..10 {
            self.data1.remove(0);
            self.data2.remove(0);

            let buy_a = Decimal::from(self.rng.gen_range(1..=30));
            let buy_b = Decimal::from(self.rng.gen_range(1..=30));

            self.amm.swap_a(buy_a);
            self.amm.swap_b(buy_b);

            let price_a = self.amm.price_a(dec!(1)).round_dp(2);
            let price_b = self.amm.price_b(dec!(1)).round_dp(2);

            self.data1.push((self.x, price_a.to_f64().unwrap()));
            self.data2.push((self.x, price_b.to_f64().unwrap()));

            self.x += 0.1;
        }

        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(size);
    let x_labels = vec![
        Span::styled(
            format!("{}", app.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
        Span::styled(
            format!("{}", app.window[1]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let datasets = vec![
        Dataset::default()
            .name("$UGAR")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&app.data1),
        Dataset::default()
            .name("CUB$")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&app.data2),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "PIDMM - Liquidity Pool",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                ])
                .bounds([0.0, 20.0]),
        );
    f.render_widget(chart, chunks[0]);
}
