use log::{self, Level, LevelFilter, Log, Metadata, Record};

use crate::println;
struct SimpleLogger;
impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => 31, //Red
            Level::Warn => 93,  //BrightYellow
            Level::Info => 34,  //Blue
            Level::Debug => 32, //Green
            Level::Trace => 90, //BrightBlack
        };

        //\u{1B} 表示一个 Unicode 转义序列，用于生成 ASCII 的 "Escape" 字符
        //[{:>5}] 是一个格式化字符串的占位符，用来控制输出的对齐方式和宽度。
        /*
          : 表示开始格式化选项
          > 表示右对齐。
          5 表示字段的最小宽度为 5 个字符。
        */
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m", //"\x1b[31mhello world\x1b[0m"
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}
