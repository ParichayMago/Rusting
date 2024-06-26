mod api;

use actix_web::{HttpServer, App, web::Data, middleware::Logger};


#[actix_web::main]
async fn main()-> std::io::Result<()>{
    std::env::set_var("Rust_log", "debug");
    std::env::set_var("Rust_backend", "1");
    env_logger::init();


    HttpServer::new(move || {
        let logger= Logger::default();
        App::new()
        .wrap(logger)
        .service(get_task)
    })
    .bind(("127.0.0.1", 80))?   
    .run()
    .await
}
