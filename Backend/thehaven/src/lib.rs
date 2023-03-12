use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_sqldb::SqlDbSender;
mod models;
mod services;

use crate::services::authentication_service;

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
        let front_or_back = get_whether_front_or_backend(req);
        let db = SqlDbSender::new();
        match front_or_back.as_str() {
            "api" => {
                let controller = get_controller(req);
                match controller.as_str() {
                    "auth" => {
                        match get_auth_method(req).as_str() {
                            "login" => {
                                let response = HttpResponse {
                                    status_code: 200,
                                    body: "Login".to_string().as_bytes().to_vec(),
                                    ..Default::default()
                                };
                                Ok(response)
                            }
                            "register" => {
                                // Figure this out
                                match serde_json::from_slice(&req.body) {
                                    Ok(user) => match authentication_service::authentication_service::register_user(&_ctx, &db, user).await {
                                        Ok(response) => HttpResponse::json(response, 200),
                                        Err(e) => {
                                            let response = HttpResponse {
                                                status_code: 400,
                                                body: "Bad Request".to_string().as_bytes().to_vec(),
                                                ..Default::default()
                                            };
                                            Ok(response)
                                        }
                                    },
                                    Err(e) => {
                                        let response = HttpResponse {
                                            status_code: 400,
                                            body: "Bad Request".to_string().as_bytes().to_vec(),
                                            ..Default::default()
                                        };
                                        Ok(response)
                                    }
                                }
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

fn get_whether_front_or_backend(req: &HttpRequest) -> String {
    match req.path.split('/').nth(1) {
        Some("ui") => "ui".to_string(),
        Some("api") => "api".to_string(),
        _ => "404".to_string(),
    }
}

fn get_controller(req: &HttpRequest) -> String {
    match req.path.split('/').nth(2) {
        Some("artist") => "artist".to_string(),
        Some("auth") => "auth".to_string(),
        _ => "404".to_string(),
    }
}

fn get_auth_method(req: &HttpRequest) -> String {
    match req.path.split('/').nth(3) {
        Some("login") => "login".to_string(),
        Some("register") => "register".to_string(),
        _ => "404".to_string(),
    }
}
