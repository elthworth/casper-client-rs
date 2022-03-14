use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
    time::Duration,
};

use humantime::DurationError;
use serde::{de::Error as SerdeError, Deserialize, Deserializer, Serialize, Serializer};

use casper_types::bytesrepr::{self, ToBytes};

/// A time difference between two timestamps.
#[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct TimeDiff(u64);

impl TimeDiff {
    /// Returns a new `TimeDiff` from the specified millisecond count.
    pub const fn from_millis(millis: u64) -> Self {
        TimeDiff(millis)
    }
}

impl Display for TimeDiff {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            humantime::format_duration(Duration::from_millis(self.0))
        )
    }
}

impl FromStr for TimeDiff {
    type Err = DurationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let inner = humantime::parse_duration(value)?.as_millis() as u64;
        Ok(TimeDiff(inner))
    }
}

impl Serialize for TimeDiff {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for TimeDiff {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            let value_as_string = String::deserialize(deserializer)?;
            TimeDiff::from_str(&value_as_string).map_err(SerdeError::custom)
        } else {
            let inner = u64::deserialize(deserializer)?;
            Ok(TimeDiff(inner))
        }
    }
}

impl ToBytes for TimeDiff {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        self.0.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        self.0.to_bytes()
    }

    fn serialized_length(&self) -> usize {
        self.0.serialized_length()
    }
}
