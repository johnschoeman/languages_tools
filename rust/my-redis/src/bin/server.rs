use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use std::{thread, time::Duration};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });

        // process(socket).await;
    }
}

async fn process(socket: TcpStream, db: Db) {
    // thread::sleep(Duration::from_secs(5));
    use mini_redis::Command::{self, Get, Set};
    // use std::collections::HashMap;
    // let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    // // No Spawning
    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }

    // // No Shared State Between Connections
    // while let Some(frame) = connection.read_frame().await.unwrap() {
    //     let response = match Command::from_frame(frame).unwrap() {
    //         Set(cmd) => {
    //             db.insert(cmd.key().to_string(), cmd.value().to_vec());
    //             Frame::Simple("OK".to_string())
    //         }
    //         Get(cmd) => {
    //             if let Some(value) = db.get(cmd.key()) {
    //                 Frame::Bulk(value.clone().into())
    //             } else {
    //                 Frame::Null
    //             }
    //         }
    //         cmd => panic!("unimplemented {:?}", cmd),
    //     };
    //
    //     connection.write_frame(&response).await.unwrap();
    // }

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
