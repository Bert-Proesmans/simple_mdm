//! Run with
//!
//! ```not_rust
//! cargo run -p example-low-level-native-tls
//! ```

use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, MatchedPath, Request},
    http::{request::Parts, HeaderMap},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures_util::pin_mut;
use http_body_util::BodyExt;
use hyper::{body::Incoming, Method, StatusCode};
use hyper_util::rt::{TokioExecutor, TokioIo};
use microsoft_protocol::mde_v2::{discover_header::ReplyToType, DiscoverHeader};
use std::{path::PathBuf, str::FromStr, time::Duration};
use tokio::net::TcpListener;
use tokio_native_tls::{
    native_tls::{Identity, Protocol, TlsAcceptor as NativeTlsAcceptor},
    TlsAcceptor,
};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use tower_service::Service;
use tracing::{debug_span, enabled, error, info, info_span, warn, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use xsd_primitives::Decimal;
use yaserde::ser::Config;

mod microsoft_protocol;
mod xsd_primitives;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "simple_mdm=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let tls_acceptor = native_tls_acceptor(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
    );

    let tls_acceptor = TlsAcceptor::from(tls_acceptor);
    let bind = "127.0.1.167:3000";
    let tcp_listener = TcpListener::bind(bind).await.unwrap();
    info!("HTTPS server listening on {bind}. To contact curl -k https://127.0.1.167:3000");
    let app = Router::new()
        .route(
            "/EnrollmentServer/Discovery.svc",
            get(get_discovery_handler).post(post_discovery_handler),
        )
        .route("/EnrollmentServer/Policy.svc", post(policy_handler))
        .route("/EnrollmentServer/Enrollment.svc", post(enroll_handler))
        .route("/ManagementServer/MDM.svc", post(manage_handler))
        .route("/", get(handler))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            1024 * 1024 * 5_000, /* 5mb */
        ))
        .layer(middleware::from_fn(trace_request_response))
        .layer(TraceLayer::new_for_http());

    pin_mut!(tcp_listener);
    loop {
        let tower_service = app.clone();
        let tls_acceptor = tls_acceptor.clone();

        // Wait for new tcp connection
        let (cnx, addr) = tcp_listener.accept().await.unwrap();

        tokio::spawn(async move {
            // Wait for tls handshake to happen
            let Ok(stream) = tls_acceptor.accept(cnx).await else {
                error!("error during tls handshake connection from {}", addr);
                return;
            };

            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let stream = TokioIo::new(stream);

            // Hyper also has its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                tower_service.clone().call(request)
            });

            let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(stream, hyper_service)
                .await;

            if let Err(err) = ret {
                warn!("error serving connection from {addr}: {err}");
            }
        });
    }
}

async fn trace_request_response(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if enabled!(Level::DEBUG) {
        let (parts, body) = req.into_parts();
        headers_print("request", &parts);
        let bytes = buffer_and_print("request", body).await?;
        req = Request::from_parts(parts, Body::from(bytes));
    }

    let mut res = next.run(req).await;

    if enabled!(Level::DEBUG) {
        let (parts, body) = res.into_parts();
        let bytes = buffer_and_print("response", body).await?;
        res = Response::from_parts(parts, Body::from(bytes));
    }

    Ok(res)
}

fn headers_print(direction: &str, parts: &Parts) {
    let ref method = parts.method;
    let ref uri = parts.uri;
    let ref headers = parts.headers;
    let http_string = format!("{method} {uri}");
    tracing::debug!("============================= {direction} HEADERS =============================\n{http_string}\n{headers:?}\n");
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("============================= {direction} body =============================\n{body}\n");
    }

    Ok(bytes)
}

async fn handler() -> &'static str {
    "Hello, World!"
}

fn native_tls_acceptor(key_file: PathBuf, cert_file: PathBuf) -> NativeTlsAcceptor {
    let key_pem = std::fs::read_to_string(&key_file).unwrap();
    let cert_pem = std::fs::read_to_string(&cert_file).unwrap();

    let id = Identity::from_pkcs8(cert_pem.as_bytes(), key_pem.as_bytes()).unwrap();
    NativeTlsAcceptor::builder(id)
        // let's be modern
        .min_protocol_version(Some(Protocol::Tlsv12))
        .build()
        .unwrap()
}

async fn get_discovery_handler() -> StatusCode {
    StatusCode::NO_CONTENT
}

// WARN; Request the payload as a string, this will consume all bytes (the body) for us
async fn post_discovery_handler(
    method: Method,
    headers: HeaderMap,
    payload: String,
) -> impl IntoResponse {
    use microsoft_protocol::mde_v2::{discover_response::*, *};

    let parsed: Result<SoapEnvelope<DiscoverRequestBody, DiscoverHeader>, _> =
        yaserde::de::from_str(&payload);

    match parsed {
        Ok(request) => {
            println!(
                "Received SOAP request with ID: {:?}",
                request.header.message_id
            );

            let response = SoapEnvelope {
                header: DiscoverResponseHeader {
                    action: "http://schemas.microsoft.com/windows/management/2012/01/enrollment/IDiscoveryService/DiscoverResponse".into(),
                    activity_id: None,
                    relates_to: request.header.message_id
                },
                body: DiscoverResponseBody {
                    discover: DiscoverResponse {
                        response: DiscoverResult {
                            auth_policy: AuthPolicyType::OnPremise,
                            enrollment_version: Some(Decimal::from_str("4.0").unwrap()),
                            // WARN; Hardcoded
                            enrollment_policy_service_url: Some("https://mdmwindows.com/EnrollmentServer/Policy.svc".into()),
                            // WARN; Hardcoded
                            enrollment_service_url: "https://mdmwindows.com/EnrollmentServer/Enrollment.svc".into(),
                            // NOTE; Only applicable for auth_policy == AuthPolicyType::Federated
                            authentication_service_url: None,
                        }
                    }
                },
                encoding_style: None,
                tnsattr: None,
                urnattr: None,
                xsiattr: None,
            };

            match yaserde::ser::to_string(&response) {
                Ok(xml) => Response::builder()
                    .header("Content-Type", "application/soap+xml; charset=utf-8")
                    .body(xml)
                    .unwrap(),
                Err(err) => {
                    eprintln!("Error serializing response: {}", err);
                    Response::builder()
                        .status(500)
                        .body("Internal Server Error".to_string())
                        .unwrap()
                }
            }
        }
        Err(err) => {
            eprintln!("Error parsing SOAP request: {}", err);
            Response::builder()
                .status(400)
                .body("Bad Request".to_string())
                .unwrap()
        }
    }
}

async fn policy_handler() {}
async fn enroll_handler() {}
async fn manage_handler() {}
