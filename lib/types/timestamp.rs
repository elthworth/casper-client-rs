use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
    time::{Duration, SystemTime},
};

use humantime::TimestampError;
use serde::{de::Error as SerdeError, Deserialize, Deserializer, Serialize, Serializer};

use casper_types::bytesrepr::{self, ToBytes};

/// A timestamp newtype, representing a specific moment in time.
#[derive(Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Returns the timestamp of the current moment.
    pub fn now() -> Self {
        let millis = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;
        Timestamp(millis)
    }
}

impl Display for Timestamp {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match SystemTime::UNIX_EPOCH.checked_add(Duration::from_millis(self.0)) {
            Some(system_time) => write!(
                formatter,
                "{}",
                humantime::format_rfc3339_millis(system_time)
            ),
            None => write!(
                formatter,
                "invalid Timestamp: {} ms after the Unix epoch",
                self.0
            ),
        }
    }
}

impl FromStr for Timestamp {
    type Err = TimestampError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let system_time = humantime::parse_rfc3339_weak(value)?;
        let inner = system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| TimestampError::OutOfRange)?
            .as_millis() as u64;
        Ok(Timestamp(inner))
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            let value_as_string = String::deserialize(deserializer)?;
            Timestamp::from_str(&value_as_string).map_err(SerdeError::custom)
        } else {
            let inner = u64::deserialize(deserializer)?;
            Ok(Timestamp(inner))
        }
    }
}

impl ToBytes for Timestamp {
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
