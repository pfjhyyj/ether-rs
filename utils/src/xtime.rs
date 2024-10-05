use anyhow::Ok;

pub const DATE: &str = "[year]-[month]-[day]";
pub const TIME: &str = "[hour]:[minute]:[second]";
pub const DATETIME: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

// get current time
pub fn now(offset: time::UtcOffset) -> time::OffsetDateTime {
    time::OffsetDateTime::now_utc().to_offset(offset)
}

// get time by string
pub fn from_str(
    fmt: &str,
    datetime: &str,
    offset: time::UtcOffset,
) -> anyhow::Result<time::OffsetDateTime> {
    let format = time::format_description::parse(fmt)?;
    let v = time::PrimitiveDateTime::parse(datetime, &format)?.assume_offset(offset);
    Ok(v)
}

// get time by timestamp
pub fn from_timestamp(
    timestamp: i64,
    offset: time::UtcOffset,
) -> anyhow::Result<time::OffsetDateTime> {
    if timestamp < 0 {
        return Ok(time::OffsetDateTime::now_utc().to_offset(offset));
    }
    let v = time::OffsetDateTime::from_unix_timestamp(timestamp)?.to_offset(offset);
    Ok(v)
}

// parse time into string
pub fn to_string(fmt: &str, timestamp: i64, offset: time::UtcOffset) -> anyhow::Result<String> {
    let format = time::format_description::parse(fmt)?;
    if timestamp < 0 {
        let v = time::OffsetDateTime::now_utc()
            .to_offset(offset)
            .format(&format)?;
        return Ok(v);
    }
    let v = time::OffsetDateTime::from_unix_timestamp(timestamp)?
        .to_offset(offset)
        .format(&format)?;
    Ok(v)
}

// parse time into timestamp
pub fn to_timestamp(fmt: &str, datetime: &str, offset: time::UtcOffset) -> anyhow::Result<i64> {
    if datetime.is_empty() {
        return Ok(0);
    }

    let format = time::format_description::parse(fmt)?;
    let v = time::PrimitiveDateTime::parse(datetime, &format)?
        .assume_offset(offset)
        .unix_timestamp();
    Ok(v)
}
