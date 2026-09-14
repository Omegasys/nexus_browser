// Nexus Browser Privacy Manager
// GPL-3.0 License


pub struct PrivacyManager {


    fingerprint_protection: bool,

    tracker_blocking: bool,

    telemetry_disabled: bool,


}



impl PrivacyManager {


    pub fn new() -> Self {


        Self {

            fingerprint_protection: true,

            tracker_blocking: true,

            telemetry_disabled: true,

        }


    }



    pub fn enable_fingerprint_protection(
        &mut self
    ) {


        self.fingerprint_protection = true;


    }



    pub fn block_trackers(
        &mut self
    ) {


        self.tracker_blocking = true;


    }



    pub fn disable_telemetry(
        &mut self
    ) {


        self.telemetry_disabled = true;


    }



    pub fn privacy_status(
        &self
    ) {


        println!(
            "Fingerprint protection: {}",
            self.fingerprint_protection
        );


        println!(
            "Tracker blocking: {}",
            self.tracker_blocking
        );


        println!(
            "Telemetry disabled: {}",
            self.telemetry_disabled
        );


    }

}
