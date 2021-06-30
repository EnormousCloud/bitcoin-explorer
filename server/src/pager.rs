use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    // token to start
    pub from: Option<String>,
    // limit of records
    pub limit: u32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            from: None,
            limit: 20,
        }
    }
}

impl Input {
    pub fn from_request(req: tide::Request<crate::State>) -> Self {
        let default_limit: u32 = 20;
        let limit = match req
            .param("limit")
            .map(|x| x.parse::<u32>().unwrap_or_default())
        {
            Ok(x) => {
                if x == 0 {
                    default_limit
                } else {
                    x
                }
            }
            Err(_) => default_limit,
        };
        let from = match req.param("from") {
            Ok(x) => {
                if x.len() > 0 {
                    Some(x.to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        Self { from, limit }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Input) -> bool {
        self.from == other.from && self.limit == other.limit
    }
}
impl Eq for Input {}
impl std::hash::Hash for Input {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.limit.hash(state);
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Output {
    // token to start the next page
    pub from: Option<String>,
}
