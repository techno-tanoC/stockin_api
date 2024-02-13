mod common;

use assert_json::assert_json;
use axum::{body::Body, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_crud() {
    let app = common::TestApp::new().await;

    let title = "example";
    let url = "https://example.com/";
    let thumbnail = "https://example.com/thumbnail.png";

    // Create
    let body = serde_json::to_string(&json!({
        "title": title,
        "url": url,
        "thumbnail": thumbnail,
    }))
    .unwrap();
    let res = app.post("/items", Body::new(body)).await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let json = common::into_json(res).await;
    assert_json!(json.clone(), {
        "data": {
            "title": title,
            "url": url,
            "thumbnail": thumbnail,
        }
    });
    let id_str = json.pointer("/data/id").unwrap().as_str().unwrap();
    let id = Uuid::try_parse(id_str).unwrap();

    // Find
    let res = app.get(&format!("/items/{}", &id_str)).await;
    assert_eq!(res.status(), StatusCode::OK);
    let json = common::into_json(res).await;
    assert_json!(json, {
        "data": {
            "id": id_str,
            "title": title,
            "url": url,
            "thumbnail": thumbnail,
        }
    });

    let title = "new-example";
    let url = "https://new-example.com/";
    let thumbnail = "https://new-example.com/thumbnail.png";

    // Update
    let body = serde_json::to_string(&json!({
        "title": title,
        "url": url,
        "thumbnail": thumbnail,
    }))
    .unwrap();
    let res = app.put(&format!("/items/{}", &id), Body::new(body)).await;
    assert_eq!(res.status(), StatusCode::OK);
    let json = common::into_json(res).await;
    assert_json!(json, {
        "data": {
            "id": id_str,
            "title": title,
            "url": url,
            "thumbnail": thumbnail,
        }
    });

    // Delete
    let res = app.delete(&format!("/items/{}", &id_str)).await;
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_index() {
    let app = common::TestApp::new().await;

    for i in 0..10 {
        let body = serde_json::to_string(&json!({
            "title": format!("example{}", i),
            "url": format!("https://example{}.com/", i),
            "thumbnail": format!("https://example{}.com/", i),
        }))
        .unwrap();
        let res = app.post("/items", Body::new(body)).await;
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    let res = app.get("/items").await;
    assert_eq!(res.status(), StatusCode::OK);
    let json = common::into_json(res).await;
    assert_json!(json, {
        "data": assert_json::validators::array_size(10),
    });
}
