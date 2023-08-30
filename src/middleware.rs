// src/middleware.rs

// use axum::extract::Extension;
// use tower::ServiceBuilder;

// use crate::app_state::AppState;

// pub fn middleware() -> ServiceBuilder<impl axum::Service<Request = _, Response = _> + Clone> {
//     ServiceBuilder::new().layer_fn(|s| {
//         let s = s.clone();
//         async move {
//             let db_pool = s
//                 .clone()
//                 .oneshot(axum::http::Request::new(())).await
//                 .unwrap()
//                 .take_extension::<AppState>()
//                 .expect("AppState missing in request extensions")
//                 .db_pool
//                 .clone();

//             let service = s.clone();
//             axum::http::Response::builder()
//                 .extension(db_pool)
//                 .body(s)
//         }
//     })
// }
