use log::LogRecord;
use chrono;
use std::io::{Write, Error};
use ::Config;

#[inline(always)]
pub fn try_log<W>(config: &Config, record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{

    if let Some(time) = config.time {
        if time <= record.level() {
            try!(write_time(write, config));
        }
    }

    if let Some(level) = config.level {
        if level <= record.level() {
            try!(write_level(record, write));
        }
    }

    if let Some(target) = config.target {
        if target <= record.level() {
            try!(write_target(record, write));
        }
    }

    if let Some(location) = config.location {
        if location <= record.level() {
            try!(write_location(record, write));
        }
    }

    try!(write_args(record, write));
    try!(write.flush());
    Ok(())
}

#[inline(always)]
pub fn write_time<W>(write: &mut W, config: &Config) -> Result<(), Error>
    where W: Write + Sized
{
    let cur_time = chrono::Utc::now();
    try!(write!(write, "{} ", cur_time.format(
            config
                .time_format
                .unwrap_or("%H:%M:%S")
    )));
    Ok(())
}

#[inline(always)]
pub fn write_level<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(write!(write, "[{}] ", record.level()));
    Ok(())
}

#[inline(always)]
pub fn write_target<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(write!(write, "{}: ", record.target()));
    Ok(())
}

#[inline(always)]
pub fn write_location<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(write!(write, "[{}:{}] ",
        record.location().file(),
        record.location().line()));
    Ok(())
}

#[inline(always)]
pub fn write_args<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(writeln!(write, "{}", record.args()));
    Ok(())
}
