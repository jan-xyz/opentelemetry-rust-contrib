//! Basic example of using OpenTelemetry AWS SDK instrumentation.

use opentelemetry_aws_sdk::{AwsOtelInterceptor, AwsOtelConfig};
use aws_config::BehaviorVersion;
use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_sdk::trace;
use opentelemetry_stdout::SpanExporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry with stdout exporter for demo purposes
    let tracer_provider = trace::TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(tracer_provider);

    // Load AWS configuration
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    // Create AWS OpenTelemetry interceptor with custom configuration
    let otel_interceptor = AwsOtelInterceptor::with_config(
        AwsOtelConfig::default()
            .with_capture_request_body(false)
            .with_capture_response_body(false)
            .with_max_attribute_length(1024)
            .with_capture_retry_info(true)
            .with_capture_error_details(true)
    );

    // Example 1: S3 operations
    println!("=== S3 Operations ===");
    let s3_client = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::Config::from(&config)
            .with_interceptor(otel_interceptor.clone())
    );

    // List buckets - this will create a span with S3-specific attributes
    match s3_client.list_buckets().send().await {
        Ok(response) => {
            println!("Found {} buckets", response.buckets().len());
            
            // If we have buckets, try to list objects in the first one
            if let Some(bucket) = response.buckets().first() {
                if let Some(bucket_name) = bucket.name() {
                    println!("Listing objects in bucket: {}", bucket_name);
                    match s3_client
                        .list_objects_v2()
                        .bucket(bucket_name)
                        .max_keys(10)
                        .send()
                        .await
                    {
                        Ok(objects_response) => {
                            println!("Found {} objects", objects_response.contents().len());
                        }
                        Err(e) => println!("Error listing objects: {}", e),
                    }
                }
            }
        }
        Err(e) => println!("Error listing buckets: {}", e),
    }

    // Example 2: DynamoDB operations
    println!("\n=== DynamoDB Operations ===");
    let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(
        aws_sdk_dynamodb::Config::from(&config)
            .with_interceptor(otel_interceptor.clone())
    );

    // List tables - this will create a span with DynamoDB-specific attributes
    match dynamodb_client.list_tables().send().await {
        Ok(response) => {
            println!("Found {} tables", response.table_names().len());
            
            // If we have tables, try to describe the first one
            if let Some(table_name) = response.table_names().first() {
                println!("Describing table: {}", table_name);
                match dynamodb_client
                    .describe_table()
                    .table_name(table_name)
                    .send()
                    .await
                {
                    Ok(describe_response) => {
                        if let Some(table) = describe_response.table() {
                            println!("Table status: {:?}", table.table_status());
                        }
                    }
                    Err(e) => println!("Error describing table: {}", e),
                }
            }
        }
        Err(e) => println!("Error listing tables: {}", e),
    }

    // Give some time for spans to be exported
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Shutdown the tracer provider to flush any remaining spans
    global::shutdown_tracer_provider();

    println!("\nExample completed! Check the console output above for OpenTelemetry spans.");
    println!("Each AWS operation should have generated a span with AWS-specific attributes.");

    Ok(())
}