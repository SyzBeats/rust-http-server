fn main() {

    let string = String::from("127.0.0.1:8080");

    // just give all the chars starting from 10. position
    let string_slice = &string[10..];
    
    dbg!(string);
    dbg!(string_slice);
    // new is an associated function
    // let server = Server::new("127.0.0.1:8080");
    // server.run();
}

// values of struct are polaces next to each other in memory
struct Server {
    address: String,
}

// implementation block for structs
// this contains all functionality of the server struct
impl Server {

    // methods are defined in the context of a struct
    // they take a special param "self" which represents the instance of the struct

    // associated function. "Self" is as uppercase just is an alias to the struct name (Server) 
    fn new(address: String) -> Self {
        Server {
            address
        }
    }

    // run takes ownership of the entire struct.
    // it will be deallocated when run exits
    fn run(self) {

    }
}
