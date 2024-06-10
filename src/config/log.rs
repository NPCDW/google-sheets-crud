use time::UtcOffset;
use tracing_subscriber::{filter::LevelFilter, Layer, prelude::*, fmt::time::OffsetTime};

pub fn init() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),
    );
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_writer(std::io::stderr)
                .with_timer(local_time.clone())
                .with_filter(LevelFilter::INFO)
        )
        .init();
}
