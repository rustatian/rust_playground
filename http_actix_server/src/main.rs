use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_rt::System;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


fn main()  {
    some_func();
}

#[inline(always)]
fn some_func() -> std::io::Result<()> {
    let sys = System::new("http-server");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get())
            .route("/{name}", web::get().to(greet))
    }).bind("127.0.0.1:8000")?
        .workers(4).run();
    sys.run()
}