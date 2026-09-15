//! Traffic statistics monitor.


pub struct TrafficMonitor {


    bytes_sent:u64,

    bytes_received:u64,


}



impl TrafficMonitor {


    pub fn new() -> Self {

        Self {

            bytes_sent:0,

            bytes_received:0,

        }

    }



    pub fn record_sent(
        &mut self,
        bytes:u64
    ) {

        self.bytes_sent += bytes;

    }



    pub fn record_received(
        &mut self,
        bytes:u64
    ) {

        self.bytes_received += bytes;

    }



    pub fn sent(
        &self
    ) -> u64 {

        self.bytes_sent

    }



    pub fn received(
        &self
    ) -> u64 {

        self.bytes_received

    }

}



impl Default for TrafficMonitor {

    fn default() -> Self {

        Self::new()

    }

}
