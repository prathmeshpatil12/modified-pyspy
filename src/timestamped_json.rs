use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::json;

use crate::Recorder;
use anyhow::Error;
use crate::stack_trace::StackTrace;


pub struct TimestampedJson {
    samples: Vec<serde_json::Value>,
}

impl TimestampedJson {
    pub fn new() -> Self {
        Self { samples: Vec::new() }
    }
}

impl Recorder for TimestampedJson {
    fn increment(&mut self, trace: &StackTrace) -> Result<(), Error> {
        let secs= (trace.timestamp_ns) / 1_000_000_000;
        let nanos = (trace.timestamp_ns as u32) % 1_000_000_000;

        let dt = DateTime::from_timestamp(secs as i64, nanos).unwrap_or(Utc::now()); 
        self.samples.push(json!({
            "timestamp": dt.to_rfc3339_opts(SecondsFormat::Micros, true),
            "stack": trace.frames.iter().map(|f| {
                format!("{} ({}:{})", f.name, 
                    f.filename.split('/').last().unwrap_or(&f.filename),
                    f.line)
            }).collect::<Vec<_>>()
        }));
        Ok(())
    }

    fn write(&self, writer: &mut dyn std::io::Write) -> Result<(), Error> {
        serde_json::to_writer_pretty(writer, &self.samples)?;
        Ok(())
    }
}