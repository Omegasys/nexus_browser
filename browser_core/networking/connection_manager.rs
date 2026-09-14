// Nexus Connection Manager
// GPL-3.0 License


#[derive(Debug, Clone)]
pub enum Transport {


    TCP,

    UDP,

    QUIC,

    HTTP,

    HTTPS,


}



#[derive(Debug)]
pub struct Connection {


    pub host:String,

    pub port:u16,

    pub transport:Transport,


}



pub struct ConnectionManager {


    connections:
        Vec<Connection>,


}



impl ConnectionManager {


    pub fn new() -> Self {


        Self {

            connections:
                Vec::new(),

        }


    }



    pub fn open_connection(

        &mut self,

        connection:Connection

    ) {


        println!(
            "Opening {:?} connection to {}:{}",
            connection.transport,
            connection.host,
            connection.port
        );


        self.connections.push(
            connection
        );


    }



    pub fn close_all(

        &mut self

    ) {


        println!(
            "Closing all connections"
        );


        self.connections.clear();


    }



    pub fn active_connections(

        &self

    ) -> usize {


        self.connections.len()


    }


}
