use std::env;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite};
use tower_http::services::ServeDir;
use tracing::{error, info, warn};

// ── Config ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Config {
    auth_enabled: bool,
    jwt_secret: Option<String>,
    // DER-encoded Ed25519 SubjectPublicKeyInfo (44 bytes)
    jwt_public_key_der: Option<Vec<u8>>,
    websockify_url: String,
    novnc_dir: String,
    proxy_port: u16,
}

impl Config {
    fn from_env() -> Self {
        let auth_enabled = env::var("AUTH_ENABLED")
            .unwrap_or_else(|_| "true".into())
            .to_lowercase()
            != "false";

        // JWT_PUBLIC_KEY: base64-encoded raw 32-byte Ed25519 public key.
        // We wrap it into a DER SubjectPublicKeyInfo so jsonwebtoken can use it.
        let jwt_public_key_der = env::var("JWT_PUBLIC_KEY")
            .ok()
            .filter(|s| !s.is_empty())
            .and_then(|s| B64.decode(s.trim()).ok())
            .map(|raw| {
                if raw.len() == 32 {
                    ed25519_raw_to_spki(&raw)
                } else {
                    // Already DER-encoded
                    raw
                }
            });

        let jwt_secret = env::var("JWT_SECRET")
            .ok()
            .filter(|s| !s.is_empty());

        let internal_port = env::var("NOVNC_INTERNAL_PORT").unwrap_or_else(|_| "6080".into());
        let websockify_url = format!("ws://127.0.0.1:{}/", internal_port);

        let novnc_dir = env::var("NOVNC_DIR").unwrap_or_else(|_| "/opt/novnc".into());

        let proxy_port = env::var("PROXY_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080u16);

        Self {
            auth_enabled,
            jwt_secret,
            jwt_public_key_der,
            websockify_url,
            novnc_dir,
            proxy_port,
        }
    }
}

// Wrap a raw 32-byte Ed25519 public key into DER SubjectPublicKeyInfo format.
// Structure: SEQUENCE { SEQUENCE { OID 1.3.101.112 } BIT_STRING { 0x00 || key } }
fn ed25519_raw_to_spki(raw: &[u8]) -> Vec<u8> {
    let mut der = vec![
        0x30, 0x2a, // SEQUENCE (42 bytes)
        0x30, 0x05, // SEQUENCE (5 bytes)
        0x06, 0x03, 0x2b, 0x65, 0x70, // OID 1.3.101.112
        0x03, 0x21, // BIT STRING (33 bytes)
        0x00, // 0 padding bits
    ];
    der.extend_from_slice(raw);
    der
}

// ── JWT validation ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct Claims {
    sub: Option<String>,
    exp: Option<u64>,
}

fn validate_jwt(token: &str, cfg: &Config) -> Result<(), String> {
    if let Some(ref der) = cfg.jwt_public_key_der {
        let key = DecodingKey::from_ed_der(der);
        let mut val = Validation::new(Algorithm::EdDSA);
        val.validate_exp = false;
        val.required_spec_claims.clear();
        decode::<Claims>(token, &key, &val).map_err(|e| e.to_string())?;
        return Ok(());
    }

    if let Some(ref secret) = cfg.jwt_secret {
        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut val = Validation::new(Algorithm::HS256);
        val.validate_exp = false;
        val.required_spec_claims.clear();
        decode::<Claims>(token, &key, &val).map_err(|e| e.to_string())?;
        return Ok(());
    }

    Err("no JWT key configured (set JWT_SECRET or JWT_PUBLIC_KEY)".into())
}

// ── WebSocket handler ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct WsQuery {
    token: Option<String>,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsQuery>,
    State(cfg): State<Config>,
) -> Response {
    if cfg.auth_enabled {
        let token = match params.token {
            Some(ref t) => t.as_str(),
            None => {
                warn!("WebSocket upgrade rejected: missing token");
                return (StatusCode::UNAUTHORIZED, "missing token").into_response();
            }
        };
        if let Err(e) = validate_jwt(token, &cfg) {
            warn!("WebSocket upgrade rejected: {}", e);
            return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
        }
    }

    let target = cfg.websockify_url.clone();
    ws.protocols(["binary", "base64"])
        .on_upgrade(move |socket| proxy_ws(socket, target))
}

// ── WebSocket proxy ───────────────────────────────────────────────────────────

async fn proxy_ws(client: WebSocket, target_url: String) {
    let server = match connect_async(&target_url).await {
        Ok((ws, _)) => ws,
        Err(e) => {
            error!("failed to connect to websockify at {}: {}", target_url, e);
            return;
        }
    };

    let (mut client_tx, mut client_rx) = client.split();
    let (mut server_tx, mut server_rx) = server.split();

    // client → server
    let c2s = async {
        while let Some(result) = client_rx.next().await {
            let msg = match result {
                Ok(m) => m,
                Err(_) => break,
            };
            let ts_msg = match axum_to_tungstenite(msg) {
                Some(m) => m,
                None => break,
            };
            if server_tx.send(ts_msg).await.is_err() {
                break;
            }
        }
    };

    // server → client
    let s2c = async {
        while let Some(result) = server_rx.next().await {
            let msg = match result {
                Ok(m) => m,
                Err(_) => break,
            };
            let ax_msg = match tungstenite_to_axum(msg) {
                Some(m) => m,
                None => break,
            };
            if client_tx.send(ax_msg).await.is_err() {
                break;
            }
        }
    };

    tokio::select! {
        _ = c2s => {},
        _ = s2c => {},
    }
}

fn axum_to_tungstenite(msg: Message) -> Option<tungstenite::Message> {
    match msg {
        Message::Binary(b) => Some(tungstenite::Message::Binary(b)),
        Message::Text(s) => Some(tungstenite::Message::Text(s)),
        Message::Ping(b) => Some(tungstenite::Message::Ping(b)),
        Message::Pong(b) => Some(tungstenite::Message::Pong(b)),
        Message::Close(_) => None,
    }
}

fn tungstenite_to_axum(msg: tungstenite::Message) -> Option<Message> {
    match msg {
        tungstenite::Message::Binary(b) => Some(Message::Binary(b)),
        tungstenite::Message::Text(s) => Some(Message::Text(s)),
        tungstenite::Message::Ping(b) => Some(Message::Ping(b)),
        tungstenite::Message::Pong(b) => Some(Message::Pong(b)),
        tungstenite::Message::Close(_) => None,
        tungstenite::Message::Frame(_) => None,
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("RUST_LOG")
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cfg = Config::from_env();

    info!(
        auth = cfg.auth_enabled,
        port = cfg.proxy_port,
        novnc_dir = %cfg.novnc_dir,
        "desktop-proxy starting"
    );

    if cfg.auth_enabled {
        if cfg.jwt_public_key_der.is_none() && cfg.jwt_secret.is_none() {
            warn!("AUTH_ENABLED=true but neither JWT_PUBLIC_KEY nor JWT_SECRET is set — all connections will be rejected");
        }
    }

    let novnc_dir = cfg.novnc_dir.clone();
    let addr = format!("0.0.0.0:{}", cfg.proxy_port);

    // Route /websockify to the WebSocket handler; everything else → static noVNC files
    let app = Router::new()
        .route("/websockify", get(ws_handler))
        .fallback_service(ServeDir::new(novnc_dir))
        .with_state(cfg);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {}: {}", addr, e));

    info!("listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("server error");
}
