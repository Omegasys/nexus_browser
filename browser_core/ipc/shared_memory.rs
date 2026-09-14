// Nexus Shared Memory System
// GPL-3.0 License


pub struct SharedMemory {


    size:
        usize,


}



impl SharedMemory {


    pub fn new(
        size: usize
    ) -> Self {


        Self {

            size,

        }


    }



    pub fn write(
        &self,
        data: &[u8]
    ) {


        println!(
            "Writing {} bytes",
            data.len()
        );


    }



    pub fn size(
        &self
    ) -> usize {


        self.size


    }


}
