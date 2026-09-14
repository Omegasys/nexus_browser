// Nexus Site Data Manager
// GPL-3.0 License


pub struct SiteDataManager {


}



impl SiteDataManager {


    pub fn new() -> Self {


        Self {}

    }



    pub fn delete_site_data(

        &self,

        domain:String

    ) {


        println!(
            "Deleting all site data for {}",
            domain
        );


        // Deletes:
        // Cookies
        // Cache
        // IndexedDB
        // Local Storage
        // Session Storage


    }



    pub fn delete_all_data(

        &self

    ) {


        println!(
            "Deleting all browser site data"
        );


    }



    pub fn list_site_data(

        &self,

        domain:String

    ) {


        println!(
            "Listing data for {}",
            domain
        );


    }


}
