use std::sync::Arc;

use warp::http::Method;
use warp::{Filter, Rejection, Reply};

use rust_core::ports::question::QuestionPort;

use crate::controllers::question::{
    add_question, delete_question, get_question, get_question_answer_controller, get_questions,
    update_question,
};
use crate::errors::return_error;
use crate::options::GrpcClients;

/// Router for handling HTTP requests related to questions.
pub struct Router {
    question_port: Arc<dyn QuestionPort + Send + Sync + 'static>,
    server_config: Arc<GrpcClients>,
}

impl Router {
    /// Creates a new Router instance with the specified QuestionPort.
    pub fn new(
        question_port: Arc<dyn QuestionPort + Send + Sync + 'static>,
        server_config: Arc<GrpcClients>,
    ) -> Self {
        Router {
            question_port: question_port.clone(),
            server_config: server_config.clone(),
        }
    }

    /// Configures and returns the Warp filter for handling HTTP requests.
    pub fn routes(self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let store_filter = warp::any().map(move || self.question_port.clone());
        let server_config = warp::any().map(move || self.server_config.clone());
        let cors = warp::cors()
            .allow_any_origin()
            .allow_header("content-type")
            .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
        let get_questions = warp::get()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(server_config.clone())
            .and(warp::query())
            .and_then(get_questions);

        let get_question = warp::get()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(server_config.clone())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and_then(get_question);

        let add_question = warp::post()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(server_config.clone())
            .and(warp::body::json())
            .and_then(add_question);

        let update_question = warp::put()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(server_config.clone())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(update_question);

        let delete_question = warp::delete()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(server_config.clone())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and_then(delete_question);

        let get_question_answer = warp::get()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(server_config.clone())
            .and(warp::path::param::<String>())
            .and(warp::path("answer"))
            .and_then(get_question_answer_controller);

        get_questions
            .with(cors)
            .or(get_question)
            .or(delete_question)
            .or(update_question)
            .or(add_question)
            .or(get_question_answer)
            .with(warp::trace::request())
            .recover(return_error)
    }
}
