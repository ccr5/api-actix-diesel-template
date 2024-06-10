#[cfg(test)]
mod test_cash_in_controller {
    use actix_api::app::app;
    use actix_api::domain::constants::POSTGRESQL_DB_URI;
    use actix_api::domain::models::cash_in::CashIn;
    use actix_api::infrastructure::databases::postgresql::db_pool;
    use actix_web::test;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use serde_json;
    use serde_json::json;
    use std::env;
    use std::sync::Arc;
    use testcontainers::clients;
    use testcontainers::images::postgres;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    pub const API_VERSION: &str = "/api/v1";

    #[actix_web::test]
    async fn test() {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();

        let docker = clients::Cli::default();
        let postgres_node = docker.run(postgres::Postgres::default());
        let connection_string = &format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            postgres_node.get_host_port_ipv4(5432)
        );

        env::set_var(POSTGRESQL_DB_URI, connection_string);

        let pool = Arc::new(db_pool());
        pool.get()
            .unwrap()
            .run_pending_migrations(MIGRATIONS)
            .unwrap();

        let app = test::init_service(app()).await;
        let request_body = json!({
            "description": "desc",
            "status": "Ok"
        });

        // Creation test
        let resp = test::TestRequest::post()
            .uri(&format!("{}/cash_in/create", API_VERSION))
            .set_json(&request_body)
            .send_request(&app)
            .await;

        assert!(resp.status().is_success());
        let cash_in: CashIn = test::read_body_json(resp).await;
        assert_eq!(cash_in.id, 1);
        assert_eq!(cash_in.description, "desc");
        assert_eq!(cash_in.status, "Ok");

        // Get test
        let resp = test::TestRequest::get()
            .uri(&format!("{}/cash_in/{}", API_VERSION, cash_in.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let retrieved_cash_in: CashIn = test::read_body_json(resp).await;
        assert_eq!(cash_in.id, retrieved_cash_in.id);
        assert_eq!(cash_in.description, retrieved_cash_in.description);
        assert_eq!(cash_in.status, retrieved_cash_in.status);

        // // Delete test
        // let resp = test::TestRequest::delete()
        //     .uri(&format!("/todos/{}", todo.id))
        //     .send_request(&app)
        //     .await;
        // assert!(resp.status().is_success());

        // // Get all test
        // let req = test::TestRequest::get().uri("/todos").to_request();
        // let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_success());
        // let todos: ResultPaging<Todo> = test::read_body_json(resp).await;
        // assert_eq!(todos.items.len(), 1);
    }
}
