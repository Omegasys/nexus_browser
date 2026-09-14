// Nexus Tab Container System
// GPL-3.0 License


pub struct TabContainer {


    pub id: u64,

    pub workspace: String,

    pub isolated: bool,


}



impl TabContainer {


    pub fn new(
        id: u64,
        workspace: String
    ) -> Self {


        Self {

            id,

            workspace,

            isolated: true,

        }


    }



    pub fn open_tab(
        &self,
        url: String
    ) {


        println!(
            "Opening isolated tab {}",
            url
        );


    }



    pub fn destroy(
        &self
    ) {


        println!(
            "Destroying tab container {}",
            self.id
        );


    }



    pub fn is_isolated(
        &self
    ) -> bool {


        self.isolated


    }


}
