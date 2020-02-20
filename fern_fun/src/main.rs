use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn};

fn main() {
    set_up_logging();
    info!("starting simulation!");
    for i in 0..26 {
        trace!("loading: {}%, very verbose debbuging information", 4 * i);
        if 5 == i {
            debug!("this is taking so long... boooring!");
        } else if 10 == i {
            debug!("still alive! yay!");
        } else if 13 == i {
            info!("halfway there!");
        } else if 16 == i {
            debug!("*scratches nose*");
            warn!("nose is itching, continuing anyways");
        } else if 20 == i {
            debug!("uh oh");
            warn!(">nose itching intensifies");
            error!("HATCHOOO!");
            debug!("encountered minor problem, trying to recover");
            info!("gesundheit");
            debug!("recovered from minor problem, continuing");
        } else if 25 == i {
            info!("successfully loaded nothing");
            info!("have a good time!");
        }
    }
}

fn set_up_logging() {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.clone().info(Color::Green);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(log::LevelFilter::Warn)
        .level_for("pretty_colored", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    debug!("finished setting up logging! yay!");
}