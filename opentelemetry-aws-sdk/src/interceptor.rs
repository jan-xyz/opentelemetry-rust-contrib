//! AWS SDK OpenTelemetry interceptor implementation.

use std::collections::HashMap;
use std::sync::Arc;

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::{
    context::{BeforeTransmitInterceptorContextMut, FinalizerInterceptorContextMut},
    Intercept,
};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use opentelemetry::{global, trace::{Status, Tracer, Span}, KeyValue, InstrumentationScope};

use crate::attributes::AttributeBuilder;
use crate::config::AwsOtelConfig;
use crate::{INSTRUMENTATION_SCOPE_NAME, INSTRUMENTATION_SCOPE_VERSION};

/// OpenTelemetry interceptor for AWS SDK operations.
#[derive(Clone)]
pub struct AwsOtelInterceptor {
    config: AwsOtelConfig,
    attribute_builders: HashMap<String, Arc<dyn AttributeBuilder + Send + Sync>>,
}

impl std::fmt::Debug for AwsOtelInterceptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AwsOtelInterceptor")
            .field("config", &self.config)
            .field("attribute_builders", &format!("{} builders", self.attribute_builders.len()))
            .finish()
    }
}

impl AwsOtelInterceptor {
    /// Create a new AWS OpenTelemetry interceptor with default configuration.
    pub fn new() -> Self {
        Self::with_config(AwsOtelConfig::default())
    }

    /// Create a new AWS OpenTelemetry interceptor with the given configuration.
    pub fn with_config(config: AwsOtelConfig) -> Self {
        let interceptor = Self {
            config,
            attribute_builders: HashMap::new(),
        };

        // Register default attribute builders for enabled services
        #[cfg(feature = "s3")]
        interceptor.register_attribute_builder("s3", Arc::new(crate::attributes::S3AttributeBuilder));

        #[cfg(feature = "dynamodb")]
        interceptor.register_attribute_builder("dynamodb", Arc::new(crate::attributes::DynamoDbAttributeBuilder));

        #[cfg(feature = "sqs")]
        interceptor.register_attribute_builder("sqs", Arc::new(crate::attributes::SqsAttributeBuilder));

        #[cfg(feature = "sns")]
        interceptor.register_attribute_builder("sns", Arc::new(crate::attributes::SnsAttributeBuilder));

        #[cfg(feature = "lambda")]
        interceptor.register_attribute_builder("lambda", Arc::new(crate::attributes::LambdaAttributeBuilder));

        interceptor
    }

    /// Register a custom attribute builder for a service.
    pub fn register_attribute_builder(&mut self, service: &str, builder: Arc<dyn AttributeBuilder + Send + Sync>) {
        self.attribute_builders.insert(service.to_string(), builder);
    }

    /// Build span attributes for the operation.
    fn build_span_attributes(
        &self,
        service: &str,
        operation: &str,
    ) -> Vec<KeyValue> {
        let attributes = vec![
            KeyValue::new("aws.service", service.to_string()),
            KeyValue::new("aws.operation", operation.to_string()),
        ];

        // Add service-specific attributes if available
        // Note: This is simplified - in a real implementation, you'd need to
        // extract the input from the interceptor context
        if let Some(_builder) = self.attribute_builders.get(service) {
            // Service-specific attributes would be added here
            // This requires access to the operation input, which is complex
            // to extract from the interceptor context in the current AWS SDK
        }

        attributes
    }
}

impl Default for AwsOtelInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Intercept for AwsOtelInterceptor {
    fn name(&self) -> &'static str {
        "AwsOtelInterceptor"
    }

    fn modify_before_attempt_completion(
        &self,
        context: &mut FinalizerInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        // Extract service and operation names from the context
        // This is simplified - in practice, you'd need to extract this information
        // from the request metadata or configuration
        let service_name = "aws".to_string(); // Placeholder
        let operation_name = "operation".to_string(); // Placeholder

        // Create span name in the format "ServiceName.OperationName"
        let span_name = format!("{}.{}", service_name, operation_name);

        // Build span attributes
        let attributes = self.build_span_attributes(&service_name, &operation_name);

        // Create and enter span
        let tracer = global::tracer_with_scope(InstrumentationScope::builder(INSTRUMENTATION_SCOPE_NAME)
            .with_version(INSTRUMENTATION_SCOPE_VERSION)
            .build());

        let mut span = tracer
            .span_builder(span_name)
            .with_attributes(attributes)
            .start(&tracer);

        // Set span status based on response
        if let Some(response) = context.response() {
            if response.status().is_success() {
                span.set_status(Status::Ok);
            } else {
                span.set_status(Status::error(format!(
                    "AWS operation failed with status: {}",
                    response.status()
                )));
            }
        } else if context.output_or_error().is_some() {
            span.set_status(Status::error("AWS operation failed"));
        }

        // End the span
        span.end();

        Ok(())
    }

    fn modify_before_transmit(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        // Add request ID to span if available
        let request = context.request();
        if let Some(request_id) = request.headers().get("x-amz-request-id") {
            let request_id_str = request_id.to_string();
            // This would need to access the current span and add the attribute
            // For now, we'll log it
            tracing::debug!("AWS Request ID: {}", request_id_str);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interceptor_creation() {
        let interceptor = AwsOtelInterceptor::new();
        assert_eq!(interceptor.name(), "AwsOtelInterceptor");
    }

    #[test]
    fn test_interceptor_with_config() {
        let config = AwsOtelConfig::default()
            .with_capture_request_body(true)
            .with_max_attribute_length(512);
        
        let interceptor = AwsOtelInterceptor::with_config(config);
        assert!(interceptor.config.capture_request_body);
        assert_eq!(interceptor.config.max_attribute_length, 512);
    }
}