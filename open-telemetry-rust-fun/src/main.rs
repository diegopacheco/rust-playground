use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use opentelemetry::metrics::MeterProvider;
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_prometheus::exporter;
use prometheus::Registry;
use rand::Rng;
use std::sync::Arc;
use std::time::Instant;

struct AppState {
    registry: Registry,
    meter_provider: SdkMeterProvider,
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "service": "open-telemetry-rust-fun",
        "status": "running",
        "endpoints": ["/", "/health", "/work", "/metrics"]
    }))
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

async fn work(req: HttpRequest, data: web::Data<Arc<AppState>>) -> HttpResponse {
    let meter = data.meter_provider.meter("http_server");

    let request_counter = meter
        .u64_counter("http_requests")
        .with_description("Total HTTP requests")
        .build();

    let active_requests = meter
        .i64_up_down_counter("http_active_requests")
        .with_description("Currently active requests")
        .build();

    let request_duration = meter
        .f64_histogram("http_request_duration_seconds")
        .with_description("HTTP request duration in seconds")
        .build();

    let error_counter = meter
        .u64_counter("http_errors")
        .with_description("Total HTTP errors")
        .build();

    let method = req.method().to_string();
    let path = req.path().to_string();
    let attrs = vec![
        KeyValue::new("method", method),
        KeyValue::new("path", path),
    ];

    active_requests.add(1, &attrs);
    request_counter.add(1, &attrs);

    let start = Instant::now();

    let mut rng = rand::thread_rng();
    let sleep_ms = rng.gen_range(10..200);
    tokio::time::sleep(tokio::time::Duration::from_millis(sleep_ms)).await;

    let should_error = rng.gen_ratio(1, 10);
    let response = if should_error {
        error_counter.add(1, &attrs);
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "random simulated failure",
            "sleep_ms": sleep_ms
        }))
    } else {
        let result: u64 = (0..sleep_ms).sum();
        HttpResponse::Ok().json(serde_json::json!({
            "result": result,
            "sleep_ms": sleep_ms,
            "message": "work completed"
        }))
    };

    let duration = start.elapsed().as_secs_f64();
    request_duration.record(duration, &attrs);
    active_requests.add(-1, &attrs);

    response
}

async fn metrics(data: web::Data<Arc<AppState>>) -> HttpResponse {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = data.registry.gather();
    let output = encoder.encode_to_string(&metric_families).unwrap();
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let registry = Registry::new();
    let prom_exporter = exporter()
        .with_registry(registry.clone())
        .build()
        .expect("failed to build prometheus exporter");

    let meter_provider = SdkMeterProvider::builder()
        .with_reader(prom_exporter)
        .build();

    let state = Arc::new(AppState {
        registry,
        meter_provider,
    });

    println!("Server running at http://0.0.0.0:8080");
    println!("Metrics at http://0.0.0.0:8080/metrics");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/work", web::get().to(work))
            .route("/metrics", web::get().to(metrics))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
