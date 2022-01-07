// use log::{debug, error, info, trace, warn};
use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack)
        .debug(Color::BrightBlue)
        .info(Color::BrightGreen)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish({
                // TODO: if the message is on multiple lines, give each a prompt.
                format_args!(
                    "[{:<5}][{:<26}]|{}",
                    // chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    colors.color(record.level()),
                    record.target(),
                    message
                )
            })
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
