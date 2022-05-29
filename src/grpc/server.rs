
use std::rc::Rc;

use crate::{
    
    setup_grpc::{logger_grpc_service_server::LoggerGrpcService, LoggerRequest, LoggerResponse}, prelude::{Logging, Logger, Severity},
};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct GRPCService {
    logger: &'static Logger
}


impl GRPCService {
    pub fn new(logger: &'static Logger) -> Self {
        GRPCService { logger: logger }
    }
}

#[tonic::async_trait]
impl LoggerGrpcService for GRPCService {
    async fn log(
        &self,
        request: Request<LoggerRequest>,
    ) -> Result<Response<LoggerResponse>, Status> {
        println!("Got a request: {:?}", request);

       
        let data = request.get_ref();

        self.logger.log(&data.message, Severity::from(data.severity.clone()), data.app.clone().unwrap());


        let reply = LoggerResponse {
            error: false
        };

        Ok(Response::new(reply))


    }
}
