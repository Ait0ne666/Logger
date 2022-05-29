mod grpc;
mod telegram;
mod services;


extern crate chrono;


pub mod prelude {
    pub use crate::grpc::prelude::*;
    pub use crate::telegram::prelude::*;
    pub use crate::services::prelude::*;
}


use std::env;

use crate::prelude::*;

use setup_grpc::logger_grpc_service_server::LoggerGrpcServiceServer;
use tonic::{transport::Server};



pub mod setup_grpc {
    tonic::include_proto!("logger"); // The string specified here must match the proto package name
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env();




    let addr = "0.0.0.0:50051".parse()?;



    static logger:Logger = Logger {};
    let grpc = GRPCService::new(&logger);
    


    Server::builder()
        .add_service(LoggerGrpcServiceServer::new(grpc))
        .serve(addr)
        .await?;




    Ok(())
}



fn load_env() {
    let mode = env::var("APP_MODE");


    match mode  {
        Ok(m) => {
            if m != "PRODUCTION" {
                dotenv::dotenv().expect("No .env file found");
            }
        },
        Err(_) => {
            dotenv::dotenv().expect("No .env file found");
        },
    }
}