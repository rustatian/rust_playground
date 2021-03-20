use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde_json::json;
use uuid::Uuid;

async fn devices(req: HttpRequest) -> impl Responder {
    let mut related_dev_groups = vec![];
    let mut tags = vec![];

    for _ in 0..1000 {
        related_dev_groups.push(Uuid::new_v4().to_string());
    }

    for _ in 0..1000 {
        tags.push(Uuid::new_v4().to_string());
    }

    let value = json!({
        "networkId": Uuid::new_v4().to_string(),
        "relatedDeviceGroupIds": related_dev_groups,
        "tags": tags,
        "deviceType":Uuid::new_v4().to_string(),
    });

    value.to_string()
}

async fn networks(req: HttpRequest) -> impl Responder {
    json!({
    "parentId" : Uuid::new_v4().to_string(),
    })
    .to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/v3/networks/devices/{uuid}", web::get().to(devices))
            .route("api/v3/networks/{name}", web::get().to(networks))
    })
    .bind(("127.0.0.1", 3001))?
    .workers(1000)
    .run()
    .await
}
