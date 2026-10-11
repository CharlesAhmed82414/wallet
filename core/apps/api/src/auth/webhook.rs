use std::collections::HashMap;

use axum::extract::{FromRequestParts, OriginalUri};
use gem_auth::{AUTHORIZATION_HEADER, BEARER_PREFIX};
use http::StatusCode;
use http::request::Parts;
use primitives::OptionStringExt;

use crate::error::ApiError;

pub struct WebhookSecret(pub String);

impl<S: Send + Sync> FromRequestParts<S> for WebhookSecret {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let unauthorized = || ApiError::from_status(StatusCode::UNAUTHORIZED);
        let value = parts.headers.get(AUTHORIZATION_HEADER).and_then(|value| value.to_str().ok()).ok_or_else(unauthorized)?;
        value.strip_prefix(BEARER_PREFIX).non_empty().map(|secret| Self(secret.to_string())).ok_or_else(unauthorized)
    }
}

pub struct WebhookRequest {
    pub headers: HashMap<String, String>,
    pub path: String,
}

impl<S: Send + Sync> FromRequestParts<S> for WebhookRequest {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Ok(OriginalUri(uri)) = OriginalUri::from_request_parts(parts, state).await;
        let headers = parts
            .headers
            .iter()
            .filter_map(|(name, value)| value.to_str().ok().map(|value| (name.as_str().to_ascii_lowercase(), value.to_string())))
            .collect();
        Ok(Self { headers, path: uri.path().to_string() })
    }
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::routing::post;
    use http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::WebhookRequest;

    #[tokio::test]
    async fn test_webhook_request() {
        let webhooks = Router::new()
            .route("/webhooks/{kind}/{sender}/{secret}", post(|request: WebhookRequest| async move { request.path }))
            .route("/webhooks/{kind}/{sender}", post(|request: WebhookRequest| async move { request.path }));
        let router = Router::new().nest("/v1", webhooks);

        for path in ["/v1/webhooks/fiat/banxa/secret", "/v1/webhooks/fiat/banxa"] {
            let response = router.clone().oneshot(Request::post(format!("{path}?query=ignored")).body(Body::empty()).unwrap()).await.unwrap();

            assert_eq!(response.into_body().collect().await.unwrap().to_bytes(), path);
        }
    }
}
