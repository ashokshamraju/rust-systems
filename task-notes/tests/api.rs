use axum::{Router, body};
use axum::body::Body;
use axum::http::{Request, StatusCode, status};
use http_body_util::BodyExt;
use tower::ServiceExt;
use task_notes::{app_router, db};

#[tokio::test]
async fn listing_tasks_when_empty_returns_empty_array() {
    let pool = db::init_pool("sqlite::memory:").await.expect("Failed to initialise database");

    let app = app_router(pool);

    let response = app.oneshot(
                Request::builder()
                .uri("/tasks")
                .body(Body::empty())
                .unwrap()).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn creating_a_task_returns_success() {
    let pool = db::init_pool("sqlite::memory:").await.unwrap();

    let app = app_router(pool);

    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/tasks")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"title": "test","body": "test"}"#))
            .unwrap()).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();

    let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_string.contains("\"title\":\"test\""));
    assert!(body_string.contains("\"body\":\"test\""));
}

#[tokio::test]
async fn full_task_lifecycle_create_update_delete() {
    let pool = db::init_pool("sqlite::memory:").await.unwrap();

    let app = app_router(pool);
    let req = Request::builder()
            .method("POST")
            .uri("/tasks")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"title": "test","body": "test"}"#))
            .unwrap();

    let (status, body) = send(&app, req).await;        
    
    assert_eq!(status, StatusCode::OK);
    let json_data:serde_json::Value = serde_json::from_str(&body).unwrap();

    let id = json_data["id"].as_i64().unwrap();

    assert!(body.contains("\"title\":\"test\""));
    assert!(body.contains("\"body\":\"test\""));

    //DIRECT WAY WITHOUT send
    // let update_response = app.clone().oneshot(
    //     Request::builder()
    //         .method("PUT")
    //         .uri(format!("/tasks/{}", id))
    //         .header("Content-Type", "application/json")
    //         .body(Body::from(r#"{"title": "wrong","pinned": true}"#))
    //         .unwrap()).await.unwrap();
    
    // let body_bytes = update_response.into_body().collect().await.unwrap().to_bytes();

    // let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    // assert!(body_string.contains("\"title\":\"wrong\""));
    // assert!(body_string.contains("\"body\":\"test\""));
    // assert!(body_string.contains("\"pinned\":true"));
    let update_req = Request::builder()
            .method("PUT")
            .uri(format!("/tasks/{}", id))
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"title": "wrong","pinned": true}"#))
            .unwrap();
    
    let (status, body) = send(&app, update_req).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"title\":\"wrong\""));
    assert!(body.contains("\"body\":\"test\""));
    assert!(body.contains("\"pinned\":true"));


    let delete_req =  Request::builder()
            .method("DELETE")
            .uri(format!("/tasks/{}",id))
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap();
    
    let  (status, body) = send(&app, delete_req).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_empty());
 
}

async fn send(app:&Router, req:Request<Body>)-> (StatusCode, String) {
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();
    (status,body_string)
    
}