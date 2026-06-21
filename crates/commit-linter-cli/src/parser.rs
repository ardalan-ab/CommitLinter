use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitHeader {
    pub commit_type: String,
    pub scope: Option<String>,
    pub subject: String,
}

pub fn parse_header(header: &str) -> Option<CommitHeader> {
    static HEADER_RE: OnceLock<Regex> = OnceLock::new();
    let re = HEADER_RE.get_or_init(|| {
        Regex::new(r"^([A-Za-z]+)(?:\(([^)]*)\))?: (.+)$").expect("valid header regex")
    });

    let captures = re.captures(header)?;
    Some(CommitHeader {
        commit_type: captures.get(1)?.as_str().to_string(),
        scope: captures.get(2).map(|scope| scope.as_str().to_string()),
        subject: captures.get(3)?.as_str().to_string(),
    })
}
