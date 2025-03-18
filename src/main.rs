use std::io::{Read,Write};
use std::net::{TcpStream,TcpListener};





fn handele_client(mut stream: TcpStream){

    let mut buffer = [0;1024];


    //readdata from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("reading failed from client");
    
    //this line convert the data in the buffer into utf8 enccoded string
    let request =String::from_utf8_lossy(&buffer[..]);
    println!("Recibed request: {}", request);

    let response = "hello , client! ".as_bytes();

    stream.write(response).expect("Failed to write response");

}


fn main() {
    let listnner = TcpListener::bind("127.0.0.1:8080").expect("failed to bind to address");
    println!("server listning on 127.0.0.1:8080");
    for stream in listnner.incoming(){
        match stream {
            Ok(stream)=>{
                std::thread::spawn(|| handele_client(stream));
            }
            Err(e)=> {
                eprintln!("failed to estavlish connection: {}", e);
            }
        }
    }
}
