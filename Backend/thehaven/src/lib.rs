use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
mod models;

use crate::models::dtos::userdto;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct TheHavenActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for TheHavenActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let frontOrBack = getWhetherFrontOrBackend(req);
        match frontOrBack.as_str() {
            "api" => {
                let controller = getController(req);
                match controller.as_str() {
                    "auth" => {
                        let response = HttpResponse {
                            status_code: 200,
                            body: "Auth".to_string().as_bytes().to_vec(),
                            ..Default::default()
                        };
                        
                        Ok(response)
                    }
                    _ => {
                        let response = HttpResponse {
                            status_code: 404,
                            body: "Not Found".to_string().as_bytes().to_vec(),
                            ..Default::default()
                        };
                        Ok(response)
                    }
                }
            }
            "ui" => {
                let response = HttpResponse {
                    status_code: 200,
                    body: "UI".to_string().as_bytes().to_vec(),
                    ..Default::default()
                };
                Ok(response)
            }
            _ => {
                let response = HttpResponse {
                    status_code: 404,
                    body: "Not Found".to_string().as_bytes().to_vec(),
                    ..Default::default()
                };
                Ok(response)
            }
        }
            
        }
    }



fn getWhetherFrontOrBackend(req: &HttpRequest) -> String {
    match req.path.split('/').nth(1) {
        Some("ui") => "ui".to_string(),
        Some("api") => "api".to_string(),
        _ => "404".to_string(),
    }
}

fn getController(req: &HttpRequest) -> String {
    match req.path.split('/').nth(2) {
        Some("artist") => "artist".to_string(),
        Some("auth") => "auth".to_string(),
        _ => "404".to_string(),
    }
}

