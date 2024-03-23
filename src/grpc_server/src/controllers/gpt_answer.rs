use grpc_interface::interfaces::gpt_answer::gpt_answer::gpt_answer_service_server::{
    GptAnswerService, GptAnswerServiceServer,
};
use grpc_interface::interfaces::gpt_answer::gpt_answer::{GetAnswerPayload, GetAnswerResponse};
use rust_core::common::errors::CoreError;
use tonic::{transport::Server, Request, Response, Status};

use crate::options::Options;

#[derive(Debug, Default)]
pub struct GptAnswerServer;

#[tonic::async_trait]
impl GptAnswerService for GptAnswerServer {
    async fn get_answer(
        &self,
        request: Request<GetAnswerPayload>,
    ) -> Result<Response<GetAnswerResponse>, Status> {
        let payload = request.into_inner();
        // TODO: Implement your logic to generate an answer based on the question.
        let answer = format!("Answer to: {}", payload.question);

        let response = GetAnswerResponse { answer };
        Ok(Response::new(response))
    }
}

pub async fn init_gpt_answer_server(options: Options) {
    let gpt_answer_config = options.servers.gpt_answer_service.clone().unwrap();
    let result = gpt_answer_config.url.parse().map_err(|err| {
        eprintln!("Error: {:?}", err);
        CoreError::InternalError
    });

    if result.is_ok() {
        let addr = result.unwrap();

        println!("GPT Answer server config at {}", addr);

        let gpt_answer_server = GptAnswerServer::default();

        Server::builder()
            .add_service(GptAnswerServiceServer::new(gpt_answer_server))
            .serve(addr)
            .await
            .unwrap();

        println!("GPT Answer server started at {}", addr);
    } else {
        eprintln!("GPT Answer server failed to start");
    }
}
