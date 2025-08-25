# OpenTelemetry AWS SDK Instrumentation

[![Crates.io: opentelemetry-aws-sdk](https://img.shields.io/crates/v/opentelemetry-aws-sdk.svg)](https://crates.io/crates/opentelemetry-aws-sdk)
[![Documentation](https://docs.rs/opentelemetry-aws-sdk/badge.svg)](https://docs.rs/opentelemetry-aws-sdk)

OpenTelemetry instrumentation for the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust).

This crate provides automatic tracing instrumentation for AWS SDK operations, capturing AWS-specific semantic attributes and following OpenTelemetry conventions.

## Features

- **Automatic Tracing**: Instruments AWS SDK operations with OpenTelemetry spans
- **AWS Semantic Attributes**: Captures AWS-specific attributes like service names, operation names, regions, and resource identifiers
- **Service-Specific Attributes**: Specialized attribute extraction for different AWS services (S3, DynamoDB, SQS, SNS, Lambda)
- **Error Handling**: Captures AWS-specific error codes and messages
- **Retry Information**: Tracks retry attempts and throttling

## Supported Services

- **S3**: Bucket names, object keys, multipart upload information
- **DynamoDB**: Table names, index names, consistent reads, projections
- **SQS**: Queue URLs, message counts, batch operations
- **SNS**: Topic ARNs, message attributes, batch publishing
- **Lambda**: Function names, invocation types

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
opentelemetry-aws-sdk = "0.1"
# Enable specific services
opentelemetry-aws-sdk = { version = "0.1", features = ["s3", "dynamodb"] }
# Or enable all supported services
opentelemetry-aws-sdk = { version = "0.1", features = ["all-services"] }
```

### Basic Setup

```rust
use opentelemetry_aws_sdk::AwsOtelInterceptor;
use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry
    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    opentelemetry::global::set_tracer_provider(tracer_provider);

    // Load AWS config
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    // Create S3 client with OpenTelemetry instrumentation
    let s3_client = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::Config::from(&config)
            .with_interceptor(AwsOtelInterceptor::new())
    );

    // AWS operations are now automatically traced
    let response = s3_client.list_buckets().send().await?;
    println!("Buckets: {:?}", response.buckets());

    Ok(())
}
```

### Advanced Configuration

```rust
use opentelemetry_aws_sdk::{AwsOtelInterceptor, AwsOtelConfig};

let interceptor = AwsOtelInterceptor::new()
    .with_config(
        AwsOtelConfig::default()
            .with_capture_request_body(true)
            .with_capture_response_body(false)
            .with_max_attribute_length(1024)
    );
```

## Captured Attributes

### Common AWS Attributes

- `aws.service`: AWS service name (e.g., "s3", "dynamodb")
- `aws.operation`: Operation name (e.g., "GetObject", "PutItem")
- `aws.region`: AWS region
- `aws.request_id`: AWS request ID from response
- `aws.error.code`: AWS error code (if error occurs)
- `aws.error.message`: AWS error message (if error occurs)

### Service-Specific Attributes

#### S3
- `aws.s3.bucket`: S3 bucket name
- `aws.s3.key`: S3 object key
- `aws.s3.copy_source`: Copy source for copy operations
- `aws.s3.upload_id`: Multipart upload ID
- `aws.s3.part_number`: Part number for multipart operations

#### DynamoDB
- `db.system`: Always "aws_dynamodb"
- `aws.dynamodb.table_names`: Table name(s)
- `aws.dynamodb.index_name`: Index name for queries/scans
- `aws.dynamodb.consistent_read`: Whether consistent read was used
- `aws.dynamodb.projection`: Projection expression
- `aws.dynamodb.limit`: Query/scan limit

#### SQS
- `messaging.system`: Always "aws_sqs"
- `messaging.destination.name`: Queue name
- `messaging.operation.type`: "send", "receive", or "delete"
- `messaging.batch.message_count`: Number of messages in batch operations
- `server.address`: Queue URL

#### SNS
- `messaging.system`: Always "aws_sns"
- `messaging.destination.name`: Topic name
- `messaging.operation.type`: "send"
- `messaging.operation.name`: Specific operation name
- `messaging.batch.message_count`: Number of messages in batch operations

## Examples

See the [examples](examples/) directory for complete examples of using this instrumentation with different AWS services.

## Minimum Supported Rust Version (MSRV)

This crate requires Rust 1.75 or later.

## License

This project is licensed under the Apache License, Version 2.0.