// Nexus Cache Manager
// GPL-3.0 License


use std::collections::HashMap;


pub struct CacheEntry {

    pub url: String,

    pub data_size: usize,

}



pub struct CacheManager {


    cache:
        HashMap<String, CacheEntry>,


}



impl CacheManager {


    pub fn new() -> Self {

        Self {

            cache:
                HashMap::new(),

        }

    }



    pub fn store(

        &mut self,

        url: String,

        size: usize

    ) {


        println!(
            "Caching {}",
            url
        );


        self.cache.insert(

            url.clone(),

            CacheEntry {

                url,

                data_size: size,

            }

        );


    }



    pub fn retrieve(

        &self,

        url: &str

    ) -> Option<&CacheEntry> {


        self.cache.get(url)


    }



    pub fn delete(

        &mut self,

        url: &str

    ) {


        self.cache.remove(url);


    }



    pub fn clear(

        &mut self

    ) {


        self.cache.clear();


    }


}
