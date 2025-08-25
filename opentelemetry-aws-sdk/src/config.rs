//! Configuration for AWS OpenTelemetry instrumentation.

/// Configuration for AWS OpenTelemetry instrumentation.
#[derive(Debug, Clone)]
pub struct AwsOtelConfig {
    /// Whether to capture request body in spans.
    pub capture_request_body: bool,
    /// Whether to capture response body in spans.
    pub capture_response_body: bool,
    /// Maximum length for attribute values (0 = no limit).
    pub max_attribute_length: usize,
    /// Whether to capture retry information.
    pub capture_retry_info: bool,
    /// Whether to capture error details.
    pub capture_error_details: bool,
}

impl Default for AwsOtelConfig {
    fn default() -> Self {
        Self {
            capture_request_body: false,
            capture_response_body: false,
            max_attribute_length: 1024,
            capture_retry_info: true,
            capture_error_details: true,
        }
    }
}

impl AwsOtelConfig {
    /// Create a new configuration builder.
    pub fn builder() -> AwsOtelConfigBuilder {
        AwsOtelConfigBuilder::default()
    }

    /// Set whether to capture request body.
    pub fn with_capture_request_body(mut self, capture: bool) -> Self {
        self.capture_request_body = capture;
        self
    }

    /// Set whether to capture response body.
    pub fn with_capture_response_body(mut self, capture: bool) -> Self {
        self.capture_response_body = capture;
        self
    }

    /// Set maximum attribute length.
    pub fn with_max_attribute_length(mut self, length: usize) -> Self {
        self.max_attribute_length = length;
        self
    }

    /// Set whether to capture retry information.
    pub fn with_capture_retry_info(mut self, capture: bool) -> Self {
        self.capture_retry_info = capture;
        self
    }

    /// Set whether to capture error details.
    pub fn with_capture_error_details(mut self, capture: bool) -> Self {
        self.capture_error_details = capture;
        self
    }
}

/// Builder for [`AwsOtelConfig`].
#[derive(Debug, Default)]
pub struct AwsOtelConfigBuilder {
    config: AwsOtelConfig,
}

impl AwsOtelConfigBuilder {
    /// Set whether to capture request body.
    pub fn with_capture_request_body(mut self, capture: bool) -> Self {
        self.config.capture_request_body = capture;
        self
    }

    /// Set whether to capture response body.
    pub fn with_capture_response_body(mut self, capture: bool) -> Self {
        self.config.capture_response_body = capture;
        self
    }

    /// Set maximum attribute length.
    pub fn with_max_attribute_length(mut self, length: usize) -> Self {
        self.config.max_attribute_length = length;
        self
    }

    /// Set whether to capture retry information.
    pub fn with_capture_retry_info(mut self, capture: bool) -> Self {
        self.config.capture_retry_info = capture;
        self
    }

    /// Set whether to capture error details.
    pub fn with_capture_error_details(mut self, capture: bool) -> Self {
        self.config.capture_error_details = capture;
        self
    }

    /// Build the configuration.
    pub fn build(self) -> AwsOtelConfig {
        self.config
    }
}