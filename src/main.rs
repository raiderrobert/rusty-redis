use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::string::{String};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("server is running on 127.0.0.1:8080...");
 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(_e) => {}
        }


    }
}

fn handle_client(mut stream: TcpStream) {
    {
        let mut reader = BufReader::new(&stream);
        let mut buf = String::new();
        while reader.read_line(&mut buf).unwrap_or(0) > 0 {
            {
                let line = buf.trim_right();

                let commands: Vec<&str> = line.split(' ').collect();
                parse_command(commands[0].to_string());
                println!("{}", line);
            }
            buf.clear();
        }
    }
    
    let response = "+OK\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_command(command: String) -> fn(){
    match String::from_str(command) {
        "GET" => get

    }
}

fn get(){

}