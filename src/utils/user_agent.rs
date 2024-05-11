//! # User agent parsing
//! To make user agent string more readable, it is parsed.
//! It is done using regexes, don't forget to download them first.

use uaparser::{UserAgentParser, Parser};
use once_cell::sync::Lazy;

static PARSER: Lazy<UserAgentParser> = Lazy::new(|| {
    UserAgentParser::from_yaml("./regexes.yaml").unwrap()
});

/// Format: `$Browser on $OS`\
/// Example: `Safari on Mac OS X`
pub fn parse_user_agent(user_agent: &str) -> String {
    let client = PARSER.parse(user_agent);
    format!("{} on {}", client.user_agent.family, client.os.family)
}