mod socket;
mod window;
mod inputs;
mod layer;
mod hypr;


use std::{io::Write, sync::Arc};

use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main(flavor="current_thread")]
async fn main() {


   let rx = tokio_util::sync::CancellationToken::new();


   let cloned = rx.clone();

   let task = tokio::spawn( layer::run(cloned, None));

   let stdin = tokio::io::stdin();
   let mut reader = BufReader::new(stdin);
   let mut line = String::new();

   print!("input: ");
   std::io::stdout().flush().unwrap();
   reader.read_line(&mut line).await.unwrap();
   println!("sending kill signal");
   rx.cancel();

   task.await.unwrap().unwrap();
   println!("shell exited");


   // inputs::run().await.unwrap();
}
