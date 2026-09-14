// Nexus Multi-Layer Kill Switch
// GPL-3.0 License


#[derive(Debug)]
pub enum KillSwitchLayer {


    VPN,

    Tor,

    DNS,

    HTTPS,

    TCP,

    UDP,

    IPv4,

    IPv6,

    All,


}



pub struct KillSwitch {


    enabled: bool,


    protected_layers:
        Vec<KillSwitchLayer>,


}



impl KillSwitch {


    pub fn new() -> Self {


        Self {


            enabled: false,

            protected_layers:
                Vec::new(),


        }


    }



    pub fn enable(

        &mut self

    ) {


        println!(
            "Network kill switch enabled"
        );


        self.enabled = true;


    }



    pub fn disable(

        &mut self

    ) {


        self.enabled = false;


    }



    pub fn protect(

        &mut self,

        layer:KillSwitchLayer

    ) {


        self.protected_layers.push(
            layer
        );


    }



    pub fn should_block(

        &self,

        layer:&KillSwitchLayer

    ) -> bool {


        self.enabled
        &&
        self.protected_layers
            .iter()
            .any(
                |item|
                std::mem::discriminant(item)
                ==
                std::mem::discriminant(layer)
            )


    }



}
