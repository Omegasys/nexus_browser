// Nexus IP Filter
// GPL-3.0 License


#[derive(Debug)]
pub enum IPVersion {


    IPv4,

    IPv6,


}



pub struct IPFilter {


    allow_ipv4: bool,

    allow_ipv6: bool,


}



impl IPFilter {


    pub fn new() -> Self {


        Self {


            allow_ipv4: true,

            allow_ipv6: true,


        }


    }



    pub fn enable_ipv4(

        &mut self

    ) {


        self.allow_ipv4 = true;


    }



    pub fn disable_ipv4(

        &mut self

    ) {


        self.allow_ipv4 = false;


    }



    pub fn enable_ipv6(

        &mut self

    ) {


        self.allow_ipv6 = true;


    }



    pub fn disable_ipv6(

        &mut self

    ) {


        self.allow_ipv6 = false;


    }



    pub fn allowed(

        &self,

        version:IPVersion

    ) -> bool {


        match version {


            IPVersion::IPv4 =>
                self.allow_ipv4,


            IPVersion::IPv6 =>
                self.allow_ipv6,


        }


    }


}
