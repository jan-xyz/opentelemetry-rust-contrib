//! OpenTelemetry instrumentation for AWS SDK for Rust
//!
//! This crate provides automatic tracing instrumentation for AWS SDK operations,
//! capturing AWS-specific semantic attributes and following OpenTelemetry conventions.
//!
//! # Features
//!
//! - **Automatic Tracing**: Instruments AWS SDK operations with OpenTelemetry spans
//! - **AWS Semantic Attributes**: Captures AWS-specific attributes like service names, operation names, regions, and resource identifiers
//! - **Service-Specific Attributes**: Specialized attribute extraction for different AWS services (S3, DynamoDB, SQS, SNS, Lambda)
//! - **Error Handling**: Captures AWS-specific error codes and messages
//! - **Retry Information**: Tracks retry attempts and throttling
//!
//! # Usage
//!
//! ```rust,ignore
//! # // This example requires aws-config and aws-sdk-s3 dependencies
//! # use opentelemetry_aws_sdk::AwsOtelInterceptor;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize OpenTelemetry
//! let tracer_provider = opentelemetry_sdk::trace::TracerProvider::default();
//! opentelemetry::global::set_tracer_provider(tracer_provider);
//!
//! // Create AWS OpenTelemetry interceptor
//! let interceptor = AwsOtelInterceptor::new();
//!
//! // The interceptor can be added to AWS SDK clients
//! // (requires aws-config and aws-sdk-* dependencies)
//! # Ok(())
//! # }
//! ```

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::broken_intra_doc_links))]

mod config;
mod interceptor;
mod attributes;

pub use config::{AwsOtelConfig, AwsOtelConfigBuilder};
pub use interceptor::AwsOtelInterceptor;
pub use attributes::*;

/// The instrumentation scope name used for all spans created by this crate.
pub const INSTRUMENTATION_SCOPE_NAME: &str = "opentelemetry-aws-sdk";

/// The instrumentation scope version.
pub const INSTRUMENTATION_SCOPE_VERSION: &str = env!("CARGO_PKG_VERSION");