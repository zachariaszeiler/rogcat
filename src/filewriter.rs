// Copyright © 2016 Felix Obenhuber
// This program is free software. It comes without any warranty, to the extent
// permitted by applicable law. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License, Version 2, as
// published by Sam Hocevar. See the COPYING file for more details.

use std::fs::File;
use std::io::Write;
use super::{Configuration, Event, Node, Record};

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(configuration: &Configuration) -> FileWriter {
        FileWriter {
            file: File::create(configuration.outputs.file.unwrap()).unwrap_or_else(|e| {
                println!("Failed to open {}", e);
                ::std::process::exit(0)
            }),
        }
    }

    fn write(&mut self, record: Record) {
        let timestamp: String = ::time::strftime("%m-%d %H:%M:%S.%f", &record.timestamp).unwrap();
        let line = format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\r\n",
                           timestamp,
                           record.tag,
                           record.process,
                           record.thread,
                           record.level,
                           record.message);
        match self.file.write(&line.into_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
}

impl Node for FileWriter {
    fn on_event(&mut self, event: Event) {
        match event {
            Event::Init => (),
            Event::Shutdown => {
                self.file.sync_all().ok();
                ()
            }
            Event::Record(r) => self.write(r),
        }
    }
}
