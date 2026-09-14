// Nexus Browser Logging System
// GPL-3.0 License

use std::time::SystemTime;


#[derive(Debug, Clone)]
pub enum LogLevel {

    Debug,

    Info,

    Warning,

    Error,

    Security,

    Privacy,

}



#[derive(Debug)]
pub struct LogEntry {


    pub level: LogLevel,

    pub message: String,

    pub timestamp: SystemTime,


}



pub struct LoggingSystem {


    entries: Vec<LogEntry>,


    enabled: bool,


}



impl LoggingSystem {


    pub fn new() -> Self {


        Self {

            entries: Vec::new(),

            enabled: true,

        }


    }



    pub fn write(
        &mut self,
        level: LogLevel,
        message: String
    ) {


        if !self.enabled {

            return;

        }


        let entry = LogEntry {


            level,

            message,

            timestamp:
                SystemTime::now(),


        };


        println!(
            "{:?}: {}",
            entry.level,
            entry.message
        );


        self.entries.push(
            entry
        );


    }



    pub fn security_event(
        &mut self,
        event: String
    ) {


        self.write(
            LogLevel::Security,
            event
        );


    }



    pub fn privacy_event(
        &mut self,
        event: String
    ) {


        self.write(
            LogLevel::Privacy,
            event
        );


    }



    pub fn clear(
        &mut self
    ) {


        self.entries.clear();


    }



    pub fn disable(
        &mut self
    ) {


        self.enabled = false;


    }


}
