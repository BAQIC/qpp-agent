use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    process::{Command, Output},
};

use axum::{
    extract::Request,
    http::{header, StatusCode},
    routing, Form, Json, RequestExt, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

pub mod data;

#[derive(Deserialize)]
pub enum Backend {
    #[serde(rename = "sv")]
    SV,
    #[serde(rename = "dm")]
    DM,
}

#[derive(Deserialize)]
pub struct QppMessage {
    pub qasm: String,
    pub shots: u32,
    pub backend: Backend,
}

async fn read_output(source: &str, message: &QppMessage) -> Value {
    if 0 == message.shots {
        match message.backend {
            Backend::SV => {
                let mut sv = data::StateVector { bases: Vec::new() };
                data::read_state(&mut sv, &format!("/tmp/{}", source)).await;
                data::print_state(&sv, &sv.probabilities()).await
            }
            Backend::DM => {
                let mut dm = data::DensityMatrix { bases: Vec::new() };
                data::read_density(&mut dm, &format!("/tmp/{}", source)).await;
                data::print_density_matrix(&dm, &dm.probabilities()).await
            }
        }
    } else {
        let mut stats = data::Statistics {
            memory: HashMap::new(),
        };
        data::read_stats(&mut stats, &format!("/tmp/{}", source)).await;
        data::print_stats(&stats).await
    }
}

async fn save_source_file(code: &str, source: &str) -> io::Result<()> {
    let mut file = File::create(source)?;
    file.write_all(code.as_bytes())
}

async fn remove_source_target_files(source: &str) -> io::Result<()> {
    fs::remove_file(&format!("/tmp/{}.qasm", source))?;
    fs::remove_file(&format!("/tmp/{}.stats", source))?;
    fs::remove_file(&format!("/tmp/{}.state", source))?;
    Ok(())
}

async fn run_program(source: &str, message: &QppMessage) -> io::Result<Output> {
    Command::new("qpp-agent")
        .arg("-s")
        .arg(message.shots.to_string())
        .arg("-f")
        .arg(&format!("/tmp/{}.qasm", source))
        .arg("--simulator")
        .arg(match message.backend {
            Backend::SV => "sv",
            Backend::DM => "dm",
        })
        .arg("-o")
        .arg(&format!("/tmp/{}", source))
        .output()
}

pub async fn consume_task(Form(message): Form<QppMessage>) -> (StatusCode, Json<Value>) {
    let source = uuid::Uuid::new_v4().to_string();
    match save_source_file(&message.qasm, &format!("/tmp/{}.qasm", source)).await {
        Ok(_) => match run_program(&source, &message).await {
            Ok(exec_output) if exec_output.status.code() == Some(0) => {
                let ret = read_output(&source, &message).await;
                match remove_source_target_files(&source).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                };
                (StatusCode::OK, Json(json!(ret)))
            }
            Ok(exec_output) => match exec_output.status.code() {
                Some(status) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        json!({"Error": format!("Got error {:?} with {:?} when execute program", String::from_utf8_lossy(&exec_output.stderr), status)}),
                    ),
                ),
                None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": "Can't get signal from execute process"})),
                ),
            },
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": format!("failed to run program: {}", e)})),
                )
            }
        },
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"Error": format!("failed to save source file: {}", e)})),
            )
        }
    }
}

pub async fn submit(request: Request) -> (StatusCode, Json<Value>) {
    match request.headers().get(header::CONTENT_TYPE) {
        Some(content_type) => match content_type.to_str().unwrap() {
            "application/x-www-form-urlencoded" => {
                let Form(message) = request.extract().await.unwrap();
                consume_task(Form(message)).await
            }
            _ => (
                StatusCode::BAD_REQUEST,
                Json(json!({"Error": format!("content type {:?} not support", content_type)})),
            ),
        },
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"Error": format!("content type not specified")})),
        ),
    }
}

#[tokio::main]
async fn main() {
    let qpp_router = Router::new().route("/submit", routing::post(submit));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, qpp_router).await.unwrap();
}
