use serde_json::Value;
use super::model;
type Event = model::Order;

#[allow(dead_code)]
pub fn parse(txt: &str) -> Option<Event> {
    let parsed: Value = serde_json::from_str(txt).ok()?;

    let event_type = parsed.get("e")
        .or_else(|| parsed.get("data").and_then(|d| d.get("e")))
        .and_then(|v| v.as_str());

    if event_type != Some("executionReport") {
        return None;
    }

    let data = parsed.get("data")?;

    serde_json::from_value::<Event>(data.clone()).ok()
}
