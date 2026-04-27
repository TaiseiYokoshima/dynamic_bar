mod socket;
mod window;
mod inputs;
mod layer;
mod hypr;

#[tokio::main(flavor="current_thread")]
async fn main() {


   layer::run().unwrap();
}
