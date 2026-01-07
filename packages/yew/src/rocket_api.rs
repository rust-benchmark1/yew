use rocket::{get, post, routes, Build, Rocket};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::http::Status;
use rocket::data::{Data, ToByteUnit};

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;

use jsonwebtoken::{encode, Header, EncodingKey};
use jsonwebtoken::decode_header;

use rand::rngs::SmallRng;
use rand::{SeedableRng, RngCore};

use wasmtime::Engine;
use std::fs::File;
use std::io::BufReader;
use std::ptr::NonNull;

use std::fs::{Permissions, set_permissions};
use std::os::unix::fs::PermissionsExt;

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};

use base64::{Engine as _, engine::general_purpose::STANDARD};

use std::collections::VecDeque;

use rustix::fs::{chmod, chown, Mode, Uid, Gid};

/// API response structure
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse {
    /// Response status
    pub status: String,
    /// Response message
    pub message: String,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    /// Subject claim
    pub sub: String,
    /// Expiration claim
    pub exp: usize,
}

#[get("/jwt/decode?<token>")]
fn refresh_token(
    // CWE 347
    //SOURCE
    token: String
) -> Json<ApiResponse> {
    // CWE 347
    //SINK
    let decoded_token  = decode_header(&token);
    let validate_token = validate_token(&decoded_token);

    if validate_token == false {
        return Json(ApiResponse {
            status: "error".to_string(),
            message: "Invalid token".to_string(),
        });
    }

    let new_token = generate_new_token();

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("generated new token: {}", new_token),
    })
}

fn validate_token(decoded: &Result<jsonwebtoken::Header, jsonwebtoken::errors::Error>) -> bool {
    match decoded {
        Ok(header) => format!("{:?}", header).len() > 10,
        Err(_) => false,
    }
}

fn generate_new_token() -> String { 
    let claims = Claims {
        sub: "user@example.com".to_string(),
        exp: 2000000000,
    };

    // CWE 330
    //SOURCE
    let mut rng = SmallRng::seed_from_u64(12345);

    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let secret_b64 = STANDARD.encode(&secret_bytes);

    // CWE 330
    //SINK
    let key = EncodingKey::from_base64_secret(&secret_b64).unwrap();

    let token = encode(&Header::default(), &claims, &key).unwrap();

    token
}

#[post("/script_exec", data = "<code>")]
fn script_exec(
    // CWE 94
    //SOURCE
    code: String
) -> Json<ApiResponse> {
    let result = Python::with_gil(|py| {
        let globals = PyDict::new(py);
        let locals = PyDict::new(py);

        // CWE 94
        //SINK
        match py.run(&code, Some(&globals), Some(&locals)) {
            Ok(()) => "script executed without error".to_string(),
            Err(e) => e.to_string(),
        }
    });

    Json(ApiResponse {
        status: "ok".to_string(),
        message: result,
    })
}

/// Loads a WASM engine from file
#[post("/engine/load", data = "<file_path>")]
pub async fn deserialize_file(
    // CWE 502
    //SOURCE
    file_path: Data<'_>
) -> Json<ApiResponse> {

    let user_input: String = file_path
        .open(1.mebibytes())
        .into_string()
        .await
        .unwrap()
        .into_inner();

    let engine = Engine::default();

    // CWE 502
    //SINK
    let module = unsafe { wasmtime::Module::deserialize_file(&engine, user_input.trim()) }.unwrap();

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("WASM module loaded: {:?}", module),
    })
}

/// Runs training epochs
#[get("/training/epochs?<epochs>&<size>")]
pub async fn training_epochs(
    // CWE 606
    //SOURCE
    epochs: u32,
    // CWE 789
    //SOURCE
    size: usize
) -> Json<ApiResponse> {
    // CWE 789
    //SINK
    let memory_size: VecDeque<u32> = VecDeque::with_capacity(size);

    if memory_size.len() != size {
        return Json(ApiResponse {
            status: "error".to_string(),
            message: "Failed to allocate requested memory size, therefore the training did not execute.".to_string(),
        });
    }

    let mut epochs_iterations: u32 = 0;
    
    let mut i = 0;
    
    // CWE 606
    //SINK
    while i < epochs {
        println!("Training epoch {}", i + 1);
        epochs_iterations += 1;
        i += 1;
    }

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("Completed {} training epochs", epochs_iterations),
    })
}

/// Builds pagination based on size
#[get("/pagination_builder?<size>")]
pub async fn pagination_builder(
    // CWE 369
    //SOURCE
    size: usize
) -> Json<ApiResponse> {
    let total_items = 1000;

    // CWE 369
    //SINK
    let number_pages = (total_items + size - 1).div_euclid(size);

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("Total pages: {}", number_pages),
    })
}

/// Changes asset access permissions
#[get("/static_permission?<path>")]
pub async fn static_permission(
    // CWE 732
    //SOURCE
    path: String
) -> Json<ApiResponse> {
    let mode = Mode::from_raw_mode(0o644);

    // CWE 732
    //SINK
    let _ = chmod(path.as_str(), mode);

    Json(ApiResponse {
        status: "ok".to_string(),
        message: "Asset permissions changed successfully".to_string(),
    })
}

/// Connects to a remote server 
#[get("/establish_connection")]
pub async fn establish_connection() -> Json<ApiResponse> {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

    // CWE 295
    //SINK
    builder.set_verify(SslVerifyMode::NONE);

    Json(ApiResponse {
        status: "ok".to_string(),
        message: format!("Connection established with remote server."),
    })
}

/// Creates Rocket instance with all routes
pub fn create_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![script_exec, refresh_token, deserialize_file, training_epochs, pagination_builder, static_permission, establish_connection])
}
