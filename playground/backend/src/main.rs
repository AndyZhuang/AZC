//! AZC Web Playground Backend
//!
//! Compiles and runs AZC code in a sandboxed environment.

use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::process::Command;
use tempfile::NamedTempFile;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/api/compile", post(compile))
        .route("/api/examples", get(examples))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("AZC Playground listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "AZC Playground API"
}

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
}

#[derive(Serialize)]
struct CompileResponse {
    success: bool,
    output: Option<String>,
    errors: Option<Vec<String>>,
    c_code: Option<String>,
}

async fn compile(Json(req): Json<CompileRequest>) -> Json<CompileResponse> {
    // Write code to temp file
    let temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            return Json(CompileResponse {
                success: false,
                output: None,
                errors: Some(vec![format!("Failed to create temp file: {}", e)]),
                c_code: None,
            });
        }
    };

    if let Err(e) = std::fs::write(temp_file.path(), &req.code) {
        return Json(CompileResponse {
            success: false,
            output: None,
            errors: Some(vec![format!("Failed to write code: {}", e)]),
            c_code: None,
        });
    }

    // Compile AZC to C
    let compile_output = Command::new("azc")
        .arg(temp_file.path())
        .output();

    let c_code = match compile_output {
        Ok(output) => {
            if output.status.success() {
                // Read generated C file
                let c_path = temp_file.path().with_extension("c");
                std::fs::read_to_string(&c_path).ok()
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Json(CompileResponse {
                    success: false,
                    output: None,
                    errors: Some(vec![stderr.to_string()]),
                    c_code: None,
                });
            }
        }
        Err(e) => {
            return Json(CompileResponse {
                success: false,
                output: None,
                errors: Some(vec![format!("Failed to run compiler: {}", e)]),
                c_code: None,
            });
        }
    };

    // Compile C to executable
    let exe_path = temp_file.path().with_extension("");
    let gcc_output = Command::new("gcc")
        .arg("-o")
        .arg(&exe_path)
        .arg(temp_file.path().with_extension("c"))
        .output();

    match gcc_output {
        Ok(output) => {
            if output.status.success() {
                // Run executable
                let run_output = Command::new(&exe_path).output();

                match run_output {
                    Ok(run) => {
                        let stdout = String::from_utf8_lossy(&run.stdout);
                        Json(CompileResponse {
                            success: run.status.success(),
                            output: Some(stdout.to_string()),
                            errors: if run.status.success() {
                                None
                            } else {
                                Some(vec![String::from_utf8_lossy(&run.stderr).to_string()])
                            },
                            c_code,
                        })
                    }
                    Err(e) => Json(CompileResponse {
                        success: false,
                        output: None,
                        errors: Some(vec![format!("Failed to run: {}", e)]),
                        c_code,
                    }),
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Json(CompileResponse {
                    success: false,
                    output: None,
                    errors: Some(vec![stderr.to_string()]),
                    c_code,
                })
            }
        }
        Err(e) => Json(CompileResponse {
            success: false,
            output: None,
            errors: Some(vec![format!("Failed to compile C: {}", e)]),
            c_code,
        }),
    }
}

#[derive(Serialize)]
struct Example {
    name: String,
    description: String,
    code: String,
}

async fn examples() -> Json<Vec<Example>> {
    Json(vec![
        Example {
            name: "Hello World".to_string(),
            description: "Basic hello world program".to_string(),
            code: r#"# Hello World in AZC

def main()
    puts "Hello, World!"
end"#.to_string(),
        },
        Example {
            name: "Fibonacci".to_string(),
            description: "Calculate Fibonacci numbers".to_string(),
            code: r#"# Fibonacci sequence

def fib(n: Int) -> Int
    if n <= 1
        n
    else
        fib(n - 1) + fib(n - 2)
    end
end

def main()
    puts "Fibonacci sequence:"
    for i in 0..10
        puts fib(i)
    end
end"#.to_string(),
        },
        Example {
            name: "Structs".to_string(),
            description: "Working with structs".to_string(),
            code: r#"# Structs in AZC

struct Point
    x: Float
    y: Float
end

impl Point
    def new(x: Float, y: Float) -> Point
        Point { x, y }
    end
    
    def distance(self) -> Float
        (self.x * self.x + self.y * self.y).sqrt()
    end
end

def main()
    let p = Point::new(3.0, 4.0)
    puts p.distance()
end"#.to_string(),
        },
        Example {
            name: "Async".to_string(),
            description: "Async functions".to_string(),
            code: r#"# Async functions in AZC

async def fetch_data() -> Future<String>
    puts "Fetching..."
    "data"
end

def main()
    puts "Async example"
end"#.to_string(),
        },
    ])
}