use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use mano_lib::machine::{Machine, MachineState};
use mano_lib::message::{Messages, Level};

#[derive(Parser)]
#[command(about = "Mano Machine Web Server")]
struct Args {
    /// Port to bind to
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

type SessionId = String;
type AppState = Arc<Mutex<HashMap<SessionId, Machine>>>;

#[derive(Serialize, Deserialize)]
struct LoadProgramRequest {
    program: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    messages: Vec<MessageEntry>,
}

#[derive(Serialize, Deserialize)]
struct MessageEntry {
    level: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct SessionInfo {
    session_id: String,
    state: MachineState,
}

fn messages_to_entries(messages: Messages) -> Vec<MessageEntry> {
    messages.entries.into_iter()
        .map(|(level, message)| MessageEntry {
            level: match level {
                Level::Info => "info".to_string(),
                Level::Error => "error".to_string(),
                Level::Debug => "debug".to_string(),
            },
            message,
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let app_state: AppState = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/session", post(create_session))
        .route("/api/session/:id", get(get_session))
        .route("/api/session/:id/load", post(load_program))
        .route("/api/session/:id/step", post(step_machine))
        .route("/api/session/:id/reset", post(reset_machine))
        .route("/api/session/:id/memory/:addr", get(get_memory))
        .layer(ServiceBuilder::new())
        .with_state(app_state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    println!("Mano Machine Web Server running on http://localhost:{}", args.port);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn create_session(State(state): State<AppState>) -> Json<ApiResponse<SessionInfo>> {
    let session_id = uuid::Uuid::new_v4().to_string();
    let machine = Machine::new();
    let machine_state = machine.get_state();

    {
        let mut sessions = state.lock().unwrap();
        sessions.insert(session_id.clone(), machine);
    }

    Json(ApiResponse {
        success: true,
        data: Some(SessionInfo {
            session_id,
            state: machine_state,
        }),
        error: None,
        messages: vec![],
    })
}

async fn get_session(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MachineState>>, StatusCode> {
    let sessions = state.lock().unwrap();
    if let Some(machine) = sessions.get(&session_id) {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(machine.get_state()),
            error: None,
            messages: vec![],
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn load_program(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<LoadProgramRequest>,
) -> Result<Json<ApiResponse<MachineState>>, StatusCode> {
    let mut sessions = state.lock().unwrap();
    if let Some(machine) = sessions.get_mut(&session_id) {
        let messages = machine.prime(request.program);
        let machine_state = machine.get_state();

        Ok(Json(ApiResponse {
            success: !messages.has_errors(),
            data: Some(machine_state),
            error: if messages.has_errors() {
                Some("Program failed to load".to_string())
            } else {
                None
            },
            messages: messages_to_entries(messages),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn step_machine(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MachineState>>, StatusCode> {
    let mut sessions = state.lock().unwrap();
    if let Some(machine) = sessions.get_mut(&session_id) {
        let messages = machine.tick();
        let machine_state = machine.get_state();

        Ok(Json(ApiResponse {
            success: !messages.has_errors(),
            data: Some(machine_state),
            error: if messages.has_errors() {
                Some("Execution error".to_string())
            } else {
                None
            },
            messages: messages_to_entries(messages),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn reset_machine(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MachineState>>, StatusCode> {
    let mut sessions = state.lock().unwrap();
    if let Some(machine) = sessions.get_mut(&session_id) {
        let messages = machine.reset();
        let machine_state = machine.get_state();

        Ok(Json(ApiResponse {
            success: true,
            data: Some(machine_state),
            error: None,
            messages: messages_to_entries(messages),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn get_memory(
    Path((session_id, addr)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<u16>>, StatusCode> {
    let sessions = state.lock().unwrap();
    if let Some(machine) = sessions.get(&session_id) {
        if let Ok(address) = u16::from_str_radix(&addr, 16) {
            let value = machine.get_memory_at_address(address);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(value),
                error: None,
                messages: vec![],
            }))
        } else {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Invalid address format".to_string()),
                messages: vec![],
            }))
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}