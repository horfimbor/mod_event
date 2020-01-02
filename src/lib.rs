use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub trait PublicEvent<'de> {
    fn from_obj(obj: impl Deserialize<'de> + Serialize) -> Self;
    fn from_json(event_type: &str, json: &str) -> Self;
    fn stream_name(&self) -> &'static str;
    fn get_json(&self) -> Result<(&'static str, String),&str>;
}