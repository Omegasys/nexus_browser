// Nexus Storage MicroVM
// GPL-3.0 License


#[derive(Debug)]
pub enum StorageType {


    Encrypted,

    Temporary,

    Persistent,

    Disposable,


}



pub struct StorageVM {


    vm_id:u64,

    storage_type:StorageType,

    encrypted:bool,


}



impl StorageVM {


    pub fn new(

        vm_id:u64,

        storage_type:StorageType

    ) -> Self {


        Self {

            vm_id,

            storage_type,

            encrypted:true,

        }


    }



    pub fn mount(

        &self

    ) {


        println!(
            "Mounting storage VM {} {:?}",
            self.vm_id,
            self.storage_type
        );


    }



    pub fn unmount(

        &self

    ) {


        println!(
            "Unmounting storage VM {}",
            self.vm_id
        );


    }



    pub fn wipe(

        &self

    ) {


        println!(
            "Securely wiping storage VM {}",
            self.vm_id
        );


    }



    pub fn encrypted(

        &self

    ) -> bool {


        self.encrypted


    }



}
