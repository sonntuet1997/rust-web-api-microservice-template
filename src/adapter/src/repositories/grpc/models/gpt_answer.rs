use rust_core::common::errors::CoreError;
use tonic::transport::Channel;

use grpc_interface::interfaces::gpt_answer::gpt_answer::{
    gpt_answer_service_client::GptAnswerServiceClient, GetAnswerPayload,
};

pub struct GptAnswerGrpcClient {
    client: GptAnswerServiceClient<Channel>,
}

impl GptAnswerGrpcClient {
    fn new(channel: Channel) -> Self {
        let client = GptAnswerServiceClient::new(channel);
        Self { client }
    }

    pub async fn get_instance(uri: &'static str) -> Result<Self, CoreError> {
        let channel = Channel::from_static(uri).connect().await.map_err(|err| {
            eprintln!("Error connecting to GPT: {:?}", err);
            CoreError::InternalError
        })?;

        let client = Self::new(channel);
        Ok(client)
    }

    pub async fn get_answer(&mut self, question: &str) -> Result<String, CoreError> {
        let request = tonic::Request::new(GetAnswerPayload {
            question: question.to_string(),
        });

        let response = self.client.get_answer(request).await.map_err(|err| {
            eprintln!("Error getting answer from GPT: {:?}", err);
            CoreError::InternalError
        })?;

        Ok(response.into_inner().answer)
    }
}
