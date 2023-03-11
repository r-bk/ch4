use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use rsdns::constants::Type;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    net::SocketAddr,
    str::FromStr,
    time::{Duration, SystemTime},
};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct EncodedTime {
    pub secs: i64,
    pub nanos: u32,
}

impl From<EncodedTime> for NaiveDateTime {
    fn from(t: EncodedTime) -> Self {
        NaiveDateTime::from_timestamp_opt(t.secs, t.nanos).expect("failed to convert a timestamp")
    }
}

impl From<SystemTime> for EncodedTime {
    fn from(st: SystemTime) -> Self {
        let dt: DateTime<Utc> = DateTime::from(st);
        Self {
            secs: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos(),
        }
    }
}

impl From<EncodedTime> for SystemTime {
    fn from(et: EncodedTime) -> Self {
        let ndt: NaiveDateTime = et.into();
        let dt: DateTime<Utc> = DateTime::from_utc(ndt, Utc);
        dt.into()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct EncodedDuration {
    pub secs: u64,
    pub nanos: u32,
}

impl From<Duration> for EncodedDuration {
    fn from(d: Duration) -> Self {
        Self {
            secs: d.as_secs(),
            nanos: d.subsec_nanos(),
        }
    }
}

impl From<EncodedDuration> for Duration {
    fn from(ed: EncodedDuration) -> Self {
        Duration::new(ed.secs, ed.nanos)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedMessage {
    pub data: String,
    pub qname: Option<String>,
    pub qtype: Option<String>,
    pub nameserver: Option<String>,
    pub timestamp: Option<EncodedTime>,
    pub duration: Option<EncodedDuration>,
}

impl EncodedMessage {
    pub fn encode(
        msg: &[u8],
        qname: Option<&str>,
        qtype: Option<Type>,
        nameserver: Option<SocketAddr>,
        ts: Option<SystemTime>,
        elapsed: Option<Duration>,
    ) -> Result<serde_json::Value> {
        let res = Self {
            data: base64::encode(msg),
            qname: qname.map(|s| s.to_string()),
            qtype: qtype.map(|t| t.to_string()),
            nameserver: nameserver.map(|ns| ns.to_string()),
            timestamp: ts.map(EncodedTime::from),
            duration: elapsed.map(EncodedDuration::from),
        };
        Ok(serde_json::to_value(res)?)
    }

    pub fn save_all(v: &[serde_json::Value], path: &str) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        serde_json::to_writer_pretty(f, v)?;
        Ok(())
    }

    pub fn load_all(path: &str) -> Result<Vec<EncodedMessage>> {
        let f = OpenOptions::new().read(true).open(path)?;
        let v: Vec<EncodedMessage> = serde_json::from_reader(f)?;
        Ok(v)
    }

    pub fn qname(&self) -> Option<&str> {
        self.qname.as_deref()
    }

    pub fn qtype(&self) -> Option<Type> {
        if let Some(ref s) = self.qtype {
            if let Ok(t) = Type::from_str(s) {
                return Some(t);
            }
        }
        None
    }

    pub fn nameserver(&self) -> Option<SocketAddr> {
        if let Some(ref ns) = self.nameserver {
            if let Ok(sa) = SocketAddr::from_str(ns) {
                return Some(sa);
            }
        }
        None
    }

    pub fn msg(&self) -> Vec<u8> {
        if let Ok(v) = base64::decode(&self.data) {
            v
        } else {
            Vec::new()
        }
    }

    pub fn time(&self) -> Option<SystemTime> {
        self.timestamp.map(|et| et.into())
    }

    pub fn elapsed(&self) -> Option<Duration> {
        self.duration.map(|d| d.into())
    }
}
