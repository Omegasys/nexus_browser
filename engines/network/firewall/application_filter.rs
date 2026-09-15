//! Application based firewall rules.


pub struct ApplicationFilter;


impl ApplicationFilter {


    pub fn allowed(
        &self,
        _application:String
    ) -> bool {

        true

    }

}
