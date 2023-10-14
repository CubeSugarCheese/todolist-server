use time::format_description::FormatItem;
use time::macros::format_description;

pub const SQL_FORMAT: &[FormatItem] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
pub const CHRONO_SQL_FORMAT: &'static str = "%Y/%m/%d %H:%M:%S";
