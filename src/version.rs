use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Version {
    V1,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = format!("{:?}", self).to_lowercase();
        f.write_str(&value)
    }
}
