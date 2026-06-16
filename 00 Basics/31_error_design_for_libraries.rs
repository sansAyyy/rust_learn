use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    // 库代码通常应该返回明确的错误类型，而不是直接 panic。
    // 调用者可以根据错误种类决定重试、提示用户、记录日志或退出程序。
    for input in ["127.0.0.1:8080", "127.0.0.1", "127.0.0.1:not-a-port", ":80"] {
        match parse_endpoint(input) {
            Ok(endpoint) => println!("parsed endpoint: {}:{}", endpoint.host, endpoint.port),
            Err(error) => {
                println!("could not parse '{input}': {error}");
                println!("debug form: {:?}", error);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Endpoint {
    host: String,
    port: u16,
}

#[derive(Debug)]
enum EndpointError {
    MissingSeparator,
    EmptyHost,
    InvalidPort(ParseIntError),
}

impl fmt::Display for EndpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EndpointError::MissingSeparator => write!(f, "expected host:port"),
            EndpointError::EmptyHost => write!(f, "host cannot be empty"),
            EndpointError::InvalidPort(error) => write!(f, "invalid port: {error}"),
        }
    }
}

impl Error for EndpointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EndpointError::InvalidPort(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ParseIntError> for EndpointError {
    fn from(value: ParseIntError) -> Self {
        EndpointError::InvalidPort(value)
    }
}

fn parse_endpoint(input: &str) -> Result<Endpoint, EndpointError> {
    let (host, port_text) = input
        .split_once(':')
        .ok_or(EndpointError::MissingSeparator)?;

    if host.is_empty() {
        return Err(EndpointError::EmptyHost);
    }

    // 因为实现了 From<ParseIntError> for EndpointError，? 可以自动转换错误类型。
    let port = port_text.parse::<u16>()?;

    Ok(Endpoint {
        host: host.to_string(),
        port,
    })
}
