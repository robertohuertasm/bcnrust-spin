use anyhow::{Context, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
    key_value::Store,
};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> Result<Response> {
    println!(
        "{:?} - {:?} - {:?}",
        req.uri().path(),
        req.method(),
        req.headers()
    );

    if req.method() == "OPTIONS" {
        return Ok(http::Response::builder()
            .status(204)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Max-Age", "86400")
            .body(None)?);
    }

    let mut router = Router::new();
    router.get("/api/v1", handle_get_root);
    router.get("/api/v1/users", handle_get_users);
    router.get("/api/v1/users/:id", handle_get_user_by_id);
    router.get("/api/v1/users/winner", handle_get_winner);
    router.post("/api/v1/users", handle_post_user);
    router.delete("/api/v1/users", handle_delete_users);
    router.handle(req)
}

fn handle_get_root(_req: Request, _params: Params) -> Result<Response> {
    Ok(http::Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .body(Some("API v1 is working nicely".into()))?)
}

fn handle_get_users(_req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let user_ids = store.get_keys().expect("failed to get keys");

    let users = user_ids
        .iter()
        .map(|id| store.get(id).expect("failed getting a known user"))
        .map(|user| serde_json::from_slice::<User>(&user).expect("failed to deserialize user"))
        .collect::<Vec<_>>();

    let users_bytes = serde_json::to_vec(&users)?;

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Some(users_bytes.into()))?)
}

fn handle_get_winner(_req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let user_ids = store.get_keys().expect("failed to get keys");
    let winner_index = rand::thread_rng().gen_range(0..user_ids.len());
    let user_id = &user_ids[winner_index];
    let user = store.get(user_id).expect("failed getting winner user");

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Some(user.into()))?)
}

fn handle_get_user_by_id(_req: Request, params: Params) -> Result<Response> {
    let user_id = params.get("id").context("expected route parameter `id`")?;

    let store = Store::open_default()?;
    let user = store.get(user_id);

    match user {
        Ok(user) => Ok(http::Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(Some(user.into()))?),
        Err(_) => Ok(http::Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body(Some("User not found".into()))?),
    }
}

fn handle_post_user(req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;

    match req.body() {
        Some(body) => {
            let user = serde_json::from_slice::<User>(body)?;
            let user_bytes = serde_json::to_vec(&user)?;
            store.set(&user.email, &user_bytes)?;
            Ok(http::Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Some(user_bytes.into()))?)
        }
        None => Ok(http::Response::builder()
            .status(400)
            .header("Access-Control-Allow-Origin", "*")
            .body(Some("No body was present in the request".into()))?),
    }
}

fn handle_delete_users(_req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let user_ids = store.get_keys().expect("failed to get keys");
    user_ids
        .iter()
        .for_each(|id| store.delete(id).expect("failed to remove user"));

    Ok(http::Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .body(Some("All users deleted".into()))?)
}
