use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};
use mormicwatch_core::{
    export::{
        recommendations::build_recommendations_json,
        status::build_status_json,
    },
    models::app_state::AppState,
};

pub fn start_http_server(state: Arc<Mutex<AppState>>) {
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:9876")
            .expect("failed to bind http server");

        println!(
            "[MorMicWatch] HTTP API listening on http://127.0.0.1:9876/status.json and http://127.0.0.1:9876/recommendations.json"
        );

        for stream in listener.incoming() {
            let Ok(mut stream) = stream else {
                continue;
            };

            let mut buffer = [0u8; 1024];
            let _ = stream.read(&mut buffer);

            let request = String::from_utf8_lossy(&buffer);

            let (status_line, content_type, body) = if request.starts_with("GET /status.json") {
                let json = {
                    let state = state.lock().unwrap();
                    build_status_json(&state)
                };

                ("HTTP/1.1 200 OK", "application/json", json)
            } else if request.starts_with("GET /recommendations.json") {
                let json = {
                    let state = state.lock().unwrap();
                    build_recommendations_json(&state)
                };

                ("HTTP/1.1 200 OK", "application/json", json)
            } else {
                (
                    "HTTP/1.1 404 NOT FOUND",
                    "application/json",
                    r#"{"error": "not found"}"#.to_string(),
                )
            };

            let response = format!(
                "{}\r\n\
Content-Type: {}\r\n\
Access-Control-Allow-Origin: *\r\n\
Cache-Control: no-cache\r\n\
Content-Length: {}\r\n\
\r\n\
{}",
                status_line,
                content_type,
                body.len(),
                body,
            );
            
            let _ = stream.write_all(response.as_bytes());
        }
    });
}