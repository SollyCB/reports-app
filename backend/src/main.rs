use tokio::{
    io::Result,
    net::{ TcpListener, TcpStream},
};
use backend::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let ip = "127.0.0.1:9000";

    let listener = TcpListener::bind(ip).await.expect("Listener Failed to Bind"); 
    println!("Connected to: {}", ip);

    loop {
        let (socket, ip) = match listener.accept().await {
            Ok((socket, ip)) => (socket, ip),
            Err(_) => {
                continue
            }
        };

        println!("{} Connected...", ip);

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) {

    println!("Got Task! Executing...");
    let mut request = Connection::new(stream).await.read_connection().await.expect("Parsnips!!").build_request().await.expect("HUUUUHHH?");

    let body = request.body();
    let headers = request.headers();
    request.write().await.unwrap();
    println!("Body: {:?},\nLen: {:?}", body, body.len());
    println!("Headers: {:?}", headers);

    // if Err { serve <CorrespondingErr>_page}
    //      else { carry on... }

}