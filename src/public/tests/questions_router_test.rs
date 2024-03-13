mod tests {
    use std::{str::FromStr, sync::Arc};

    use deadpool_diesel::{
        postgres::{Pool, Runtime},
        Manager,
    };
    use diesel_migrations::MigrationHarness;
    use rand::Rng;
    use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};
    use warp::http::StatusCode;
    use warp::test::request;

    use adapter::repositories::{
        in_memory::question::QuestionInMemoryRepository,
        postgres::question_db::{QuestionDBRepository, MIGRATIONS},
    };
    use cli::{
        options::{GrpcClients, ServiceServer},
        router::Router,
    };
    use rust_core::{
        entities::question::{QuestionEntity, QuestionId},
        ports::question::QuestionPort,
    };

    struct DatabaseConfig {
        url: String,
        max_size: usize,
    }

    async fn question_router_test<T>(question_port: Arc<T>)
    where
        T: QuestionPort + Send + Sync + 'static,
    {
        let server_config: GrpcClients = GrpcClients {
            gpt_answer_service: Some(ServiceServer {
                url: "http://localhost:50051".to_string(),
            }),
        };

        let router = Router::new(question_port, Arc::new(server_config));
        let routers = router.routes();

        let raw_question_id: String = rand::thread_rng().gen_range(1..=1000).to_string();
        let question_id = QuestionId::from_str(&raw_question_id.clone()).unwrap();

        let question = QuestionEntity {
            id: question_id.clone(),
            title: "How to write integration tests in Rust?".to_string(),
            content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
            tags: Some(vec!["rust".to_string(), "warp".to_string()]),
        };

        // Test POST /questions to create a new question
        let post_resp = request()
            .method("POST")
            .path("/questions")
            .json(&question)
            .reply(&routers)
            .await;

        assert_eq!(
            post_resp.status(),
            StatusCode::OK,
            "Failed to create a new question"
        );

        // Test GET /questions to get the created question
        let get_resp = request()
            .method("GET")
            .path(format!("/questions/{}", raw_question_id).as_str())
            .reply(&routers)
            .await;

        assert_eq!(
            get_resp.status(),
            StatusCode::OK,
            "Failed to get the created question"
        );

        // Deserialize the response body into a Value
        let get_question: QuestionEntity =
            serde_json::from_slice(get_resp.body()).expect("Failed to deserialize response body");

        assert_eq!(
            get_question, question,
            "The created question is not as expected"
        );

        // Test PUT /questions to update the created question
        let updated_question = QuestionEntity {
            id: question_id.clone(),
            title: "Testing in Rust".to_string(),
            content: "I'm writing test cases in Rust".to_string(),
            tags: Some(vec![
                "rust".to_string(),
                "warp".to_string(),
                "testing".to_string(),
            ]),
        };

        let put_resp = request()
            .method("PUT")
            .path(format!("/questions/{}", raw_question_id).as_str())
            .json(&updated_question)
            .reply(&routers)
            .await;

        assert_eq!(
            put_resp.status(),
            StatusCode::OK,
            "Failed to update the created question"
        );

        // Test GET /questions to get the updated question
        let get_updated_resp = request()
            .method("GET")
            .path(format!("/questions/{}", raw_question_id).as_str())
            .reply(&routers)
            .await;

        assert_eq!(
            get_updated_resp.status(),
            StatusCode::OK,
            "Failed to get the updated question"
        );

        // Deserialize the response body into a Value
        let get_updated_question: QuestionEntity = serde_json::from_slice(get_updated_resp.body())
            .expect("Failed to deserialize response body");

        assert_eq!(
            get_updated_question, updated_question,
            "The updated question is not as expected"
        );

        // Test DELETE /questions to delete the created question
        let delete_resp = request()
            .method("DELETE")
            .path(format!("/questions/{}", raw_question_id).as_str())
            .reply(&routers)
            .await;

        assert_eq!(
            delete_resp.status(),
            StatusCode::OK,
            "Failed to delete the created question"
        );

        // Test GET /questions to get the deleted question
        let get_deleted_resp = request()
            .method("GET")
            .path(format!("/questions/{}", raw_question_id).as_str())
            .reply(&routers)
            .await;

        assert_eq!(
            get_deleted_resp.status(),
            StatusCode::NOT_FOUND,
            "The deleted question still exists"
        );

        // Test UPDATE non-existence question
        let put_resp = request()
            .method("PUT")
            .path(format!("/questions/{}", raw_question_id).as_str())
            .json(&updated_question)
            .reply(&routers)
            .await;

        assert_eq!(
            put_resp.status(),
            StatusCode::NOT_FOUND,
            "Failed to update the non-existence question"
        );
    }

    #[tokio::test]
    async fn questions_router_postgres_test() {
        // Set up a postgres database question port for testing
        let docker = Cli::default();
        let postgres_instance = docker.run(Postgres::default());

        let database_url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres_instance.get_host_port_ipv4(5432)
        );
        let database_config = DatabaseConfig {
            url: database_url.clone(),
            max_size: 10,
        };

        let manager = Manager::new(database_config.url, Runtime::Tokio1);
        let db_pool = Pool::builder(manager)
            .max_size(database_config.max_size)
            .build()
            .unwrap();

        let conn = db_pool.get().await.unwrap();

        // Migration the database
        let result = conn
            .interact(|connection| {
                let result = MigrationHarness::run_pending_migrations(connection, MIGRATIONS);
                match result {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            })
            .await;
        assert!(result.is_ok(), "Failed to run the migrations");

        // Create a Router instance with the postgres db question port
        let question_port = Arc::new(QuestionDBRepository::new(db_pool.clone()));
        question_router_test(question_port).await;
    }

    #[tokio::test]
    async fn questions_router_in_memory_test() {
        // Create a Router instance with the in question port
        let question_port = Arc::new(QuestionInMemoryRepository::default());
        question_router_test(question_port).await;
    }
}
