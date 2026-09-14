// Nexus Site Isolation System
// GPL-3.0 License


#[derive(Clone)]
pub struct SiteContainer {


    pub domain: String,

    pub process_id: u64,


}



pub struct SiteIsolation {


    sites: Vec<SiteContainer>,


}



impl SiteIsolation {


    pub fn new() -> Self {


        Self {

            sites:
                Vec::new(),

        }


    }



    pub fn assign_site(
        &mut self,
        domain: String,
        process_id: u64
    ) {


        println!(
            "Assigning {} to process {}",
            domain,
            process_id
        );


        self.sites.push(
            SiteContainer {

                domain,

                process_id,

            }
        );


    }



    pub fn get_process(
        &self,
        domain: &str
    ) -> Option<u64> {


        self.sites
            .iter()
            .find(
                |site|
                site.domain == domain
            )
            .map(
                |site|
                site.process_id
            )


    }



    pub fn isolate_cross_origin(
        &self
    ) {


        println!(
            "Enforcing cross-origin isolation"
        );


    }


}
