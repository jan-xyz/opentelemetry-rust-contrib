# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of OpenTelemetry AWS SDK instrumentation
- Support for S3, DynamoDB, SQS, SNS, and Lambda services
- AWS-specific semantic attributes following OpenTelemetry conventions
- Automatic error code and retry information capture
- Configurable request/response body capture
- Service-specific attribute builders