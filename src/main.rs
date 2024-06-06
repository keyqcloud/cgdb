use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use tokio::runtime;

mod database;
mod models;
mod services;
mod api;
mod config;
mod utils;

use models::context::Context;
use services::context_service::ContextService;

type Db = Arc<Mutex<ContextService>>;

use hyper::body::to_bytes;
use serde::{Deserialize};

#[derive(Deserialize)]
struct NewContext {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct NewNode {
    context_id: u32,
    node_id: u32,
    label: String,
}

#[derive(Deserialize)]
struct NewEdge {
    context_id: u32,
    from: u32,
    to: u32,
    label: String,
}

async fn handle_request(req: Request<Body>, db: Db) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/add_context") => {
            let whole_body = to_bytes(req.into_body()).await?;
            let new_context: Result<NewContext, serde_json::Error> = serde_json::from_slice(&whole_body);
            match new_context {
                Ok(context) => {
                    let context = Context::new(context.id, &context.name);
                    db.lock().unwrap().add_context(context);
                    Ok(Response::new(Body::from("Context added")))
                }
                Err(_) => Ok(Response::new(Body::from("Invalid JSON"))),
            }
        }
        (&hyper::Method::POST, "/add_node") => {
            let whole_body = to_bytes(req.into_body()).await?;
            let new_node: Result<NewNode, serde_json::Error> = serde_json::from_slice(&whole_body);
            match new_node {
                Ok(node) => {
                    db.lock().unwrap().add_node_to_context(node.context_id, node.node_id, &node.label);
                    Ok(Response::new(Body::from("Node added")))
                }
                Err(_) => Ok(Response::new(Body::from("Invalid JSON"))),
            }
        }
        (&hyper::Method::POST, "/add_edge") => {
            let whole_body = to_bytes(req.into_body()).await?;
            let new_edge: Result<NewEdge, serde_json::Error> = serde_json::from_slice(&whole_body);
            match new_edge {
                Ok(edge) => {
                    db.lock().unwrap().add_edge_to_context(edge.context_id, edge.from, edge.to, &edge.label);
                    Ok(Response::new(Body::from("Edge added")))
                }
                Err(_) => Ok(Response::new(Body::from("Invalid JSON"))),
            }
        }
        (&hyper::Method::GET, path) if path.starts_with("/get_context/") => {
            let id_str = path.trim_start_matches("/get_context/");
            if let Ok(id) = id_str.parse::<u32>() {
                if let Some(context) = db.lock().unwrap().get_context(id) {
                    Ok(Response::new(Body::from(format!("{:?}", context))))
                } else {
                    Ok(Response::new(Body::from("Context not found")))
                }
            } else {
                Ok(Response::new(Body::from("Invalid ID")))
            }
        }
        (&hyper::Method::GET, path) if path.starts_with("/get_node/") => {
            let id_str = path.trim_start_matches("/get_node/");
            if let Ok(id) = id_str.parse::<u32>() {
                if let Some(node) = db.lock().unwrap().schema.get_node(id) {
                    Ok(Response::new(Body::from(format!("{:?}", node))))
                } else {
                    Ok(Response::new(Body::from("Node not found")))
                }
            } else {
                Ok(Response::new(Body::from("Invalid ID")))
            }
        }
        (&hyper::Method::GET, path) if path.starts_with("/get_edge/") => {
            let ids_str: Vec<&str> = path.trim_start_matches("/get_edge/").split('/').collect();
            if ids_str.len() == 2 {
                if let (Ok(from), Ok(to)) = (ids_str[0].parse::<u32>(), ids_str[1].parse::<u32>()) {
                    if let Some(edge) = db.lock().unwrap().schema.get_edge(from, to) {
                        Ok(Response::new(Body::from(format!("{:?}", edge))))
                    } else {
                        Ok(Response::new(Body::from("Edge not found")))
                    }
                } else {
                    Ok(Response::new(Body::from("Invalid IDs")))
                }
            } else {
                Ok(Response::new(Body::from("Invalid path format. Use /get_edge/{from}/{to}")))
            }
        }
        _ => Ok(Response::new(Body::from("Not Found"))),
    }
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(ContextService::new()));
    let make_svc = make_service_fn(move |_| {
        let db = db.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, db.clone())
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
