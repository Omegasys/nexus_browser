//! Yggdrasil IPv6 address management.

pub struct Ipv6Manager {


    address:
        Option<String>,


}



impl Ipv6Manager {


    pub fn new() -> Self {

        Self {

            address:
                None,

        }

    }



    pub fn assign(
        &mut self,
        address:String
    ) {

        self.address =
            Some(address);

    }



    pub fn address(
        &self
    ) -> Option<&String> {

        self.address.as_ref()

    }



    pub fn assigned(
        &self
    ) -> bool {

        self.address.is_some()

    }

}



impl Default for Ipv6Manager {

    fn default() -> Self {

        Self::new()

    }

}
