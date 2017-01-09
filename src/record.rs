// Copyright © 2016 Felix Obenhuber
// This program is free software. It comes without any warranty, to the extent
// permitted by applicable law. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License, Version 2, as
// published by Sam Hocevar. See the COPYING file for more details.

#[derive (Clone, Debug, PartialOrd, PartialEq)]
pub enum Level {
    None,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Assert,
}

impl ::std::fmt::Display for Level {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Level::None => "-",
                   Level::Trace => "T",
                   Level::Debug => "D",
                   Level::Info => "I",
                   Level::Warn => "W",
                   Level::Error => "E",
                   Level::Fatal => "F",
                   Level::Assert => "A",
               })
    }
}

impl Default for Level {
    fn default() -> Level {
        Level::None
    }
}

impl<'a> From<&'a str> for Level {
    fn from(s: &str) -> Self {
        match s {
            "T" | "trace" => Level::Trace,
            "I" | "info" => Level::Info,
            "W" | "warn" => Level::Warn,
            "E" | "error" => Level::Error,
            "F" | "fatal" => Level::Fatal,
            "A" | "assert" => Level::Assert,
            "D" | "debug" => Level::Debug,
            _ => Level::None,
        }
    }
}

impl ::std::str::FromStr for Level {
    type Err = bool;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Record {
    pub timestamp: Option<::time::Tm>,
    pub message: String,
    pub level: Level,
    pub tag: String,
    pub process: String,
    pub thread: String,
    pub raw: String,
}
