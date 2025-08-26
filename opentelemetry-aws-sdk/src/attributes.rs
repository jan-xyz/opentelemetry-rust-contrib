//! AWS service-specific attribute builders.

use opentelemetry::KeyValue;
use opentelemetry_semantic_conventions as semconv;

/// Trait for building service-specific attributes from AWS SDK operations.
pub trait AttributeBuilder {
    /// Build attributes for the given operation input.
    fn build_attributes(&self, service: &str, operation: &str, input: &dyn std::any::Any) -> Vec<KeyValue>;
}

/// S3-specific attribute builder.
#[cfg(feature = "s3")]
#[cfg_attr(docsrs, doc(cfg(feature = "s3")))]
pub struct S3AttributeBuilder;

#[cfg(feature = "s3")]
impl AttributeBuilder for S3AttributeBuilder {
    fn build_attributes(&self, _service: &str, operation: &str, input: &dyn std::any::Any) -> Vec<KeyValue> {
        let mut attributes = Vec::new();
        
        match operation {
            "GetObject" | "PutObject" | "DeleteObject" | "HeadObject" => {
                if let Some(get_input) = input.downcast_ref::<aws_sdk_s3::operation::get_object::GetObjectInput>() {
                    attributes.push(KeyValue::new("aws.s3.bucket", get_input.bucket().unwrap_or_default().to_string()));
                    attributes.push(KeyValue::new("aws.s3.key", get_input.key().unwrap_or_default().to_string()));
                }
            }
            "ListObjects" | "ListObjectsV2" => {
                if let Some(list_input) = input.downcast_ref::<aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Input>() {
                    attributes.push(KeyValue::new("aws.s3.bucket", list_input.bucket().unwrap_or_default().to_string()));
                    if let Some(prefix) = list_input.prefix() {
                        attributes.push(KeyValue::new("aws.s3.prefix", prefix.to_string()));
                    }
                    if let Some(max_keys) = list_input.max_keys() {
                        attributes.push(KeyValue::new("aws.s3.max_keys", max_keys as i64));
                    }
                }
            }
            "CopyObject" => {
                if let Some(copy_input) = input.downcast_ref::<aws_sdk_s3::operation::copy_object::CopyObjectInput>() {
                    attributes.push(KeyValue::new("aws.s3.bucket", copy_input.bucket().unwrap_or_default().to_string()));
                    attributes.push(KeyValue::new("aws.s3.key", copy_input.key().unwrap_or_default().to_string()));
                    attributes.push(KeyValue::new("aws.s3.copy_source", copy_input.copy_source().unwrap_or_default().to_string()));
                }
            }
            _ => {}
        }
        
        attributes
    }
}

/// DynamoDB-specific attribute builder.
#[cfg(feature = "dynamodb")]
#[cfg_attr(docsrs, doc(cfg(feature = "dynamodb")))]
pub struct DynamoDbAttributeBuilder;

#[cfg(feature = "dynamodb")]
impl AttributeBuilder for DynamoDbAttributeBuilder {
    fn build_attributes(&self, _service: &str, operation: &str, input: &dyn std::any::Any) -> Vec<KeyValue> {
        let mut attributes = vec![
            KeyValue::new(semconv::DB_SYSTEM, semconv::DB_SYSTEM_DYNAMODB),
        ];
        
        match operation {
            "GetItem" => {
                if let Some(get_input) = input.downcast_ref::<aws_sdk_dynamodb::operation::get_item::GetItemInput>() {
                    attributes.push(KeyValue::new("aws.dynamodb.table_names", get_input.table_name().unwrap_or_default().to_string()));
                    if let Some(consistent_read) = get_input.consistent_read() {
                        attributes.push(KeyValue::new("aws.dynamodb.consistent_read", consistent_read));
                    }
                }
            }
            "PutItem" => {
                if let Some(put_input) = input.downcast_ref::<aws_sdk_dynamodb::operation::put_item::PutItemInput>() {
                    attributes.push(KeyValue::new("aws.dynamodb.table_names", put_input.table_name().unwrap_or_default().to_string()));
                }
            }
            "Query" => {
                if let Some(query_input) = input.downcast_ref::<aws_sdk_dynamodb::operation::query::QueryInput>() {
                    attributes.push(KeyValue::new("aws.dynamodb.table_names", query_input.table_name().unwrap_or_default().to_string()));
                    if let Some(index_name) = query_input.index_name() {
                        attributes.push(KeyValue::new("aws.dynamodb.index_name", index_name.to_string()));
                    }
                    if let Some(consistent_read) = query_input.consistent_read() {
                        attributes.push(KeyValue::new("aws.dynamodb.consistent_read", consistent_read));
                    }
                    if let Some(limit) = query_input.limit() {
                        attributes.push(KeyValue::new("aws.dynamodb.limit", limit as i64));
                    }
                    if let Some(scan_forward) = query_input.scan_index_forward() {
                        attributes.push(KeyValue::new("aws.dynamodb.scan_forward", scan_forward));
                    }
                }
            }
            "Scan" => {
                if let Some(scan_input) = input.downcast_ref::<aws_sdk_dynamodb::operation::scan::ScanInput>() {
                    attributes.push(KeyValue::new("aws.dynamodb.table_names", scan_input.table_name().unwrap_or_default().to_string()));
                    if let Some(index_name) = scan_input.index_name() {
                        attributes.push(KeyValue::new("aws.dynamodb.index_name", index_name.to_string()));
                    }
                    if let Some(consistent_read) = scan_input.consistent_read() {
                        attributes.push(KeyValue::new("aws.dynamodb.consistent_read", consistent_read));
                    }
                    if let Some(limit) = scan_input.limit() {
                        attributes.push(KeyValue::new("aws.dynamodb.limit", limit as i64));
                    }
                }
            }
            _ => {}
        }
        
        attributes
    }
}

/// SQS-specific attribute builder.
#[cfg(feature = "sqs")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqs")))]
pub struct SqsAttributeBuilder;

#[cfg(feature = "sqs")]
impl AttributeBuilder for SqsAttributeBuilder {
    fn build_attributes(&self, _service: &str, operation: &str, input: &dyn std::any::Any) -> Vec<KeyValue> {
        let mut attributes = vec![
            KeyValue::new(semconv::MESSAGING_SYSTEM, semconv::MESSAGING_SYSTEM_AWS_SQS),
        ];
        
        match operation {
            "SendMessage" => {
                if let Some(send_input) = input.downcast_ref::<aws_sdk_sqs::operation::send_message::SendMessageInput>() {
                    attributes.push(KeyValue::new(semconv::MESSAGING_OPERATION_TYPE, semconv::MESSAGING_OPERATION_TYPE_SEND));
                    attributes.push(KeyValue::new("server.address", send_input.queue_url().unwrap_or_default().to_string()));
                    // Extract queue name from URL
                    if let Some(queue_url) = send_input.queue_url() {
                        if let Some(queue_name) = queue_url.split('/').last() {
                            attributes.push(KeyValue::new(semconv::MESSAGING_DESTINATION_NAME, queue_name.to_string()));
                        }
                    }
                }
            }
            "SendMessageBatch" => {
                if let Some(batch_input) = input.downcast_ref::<aws_sdk_sqs::operation::send_message_batch::SendMessageBatchInput>() {
                    attributes.push(KeyValue::new(semconv::MESSAGING_OPERATION_TYPE, semconv::MESSAGING_OPERATION_TYPE_SEND));
                    attributes.push(KeyValue::new("server.address", batch_input.queue_url().unwrap_or_default().to_string()));
                    attributes.push(KeyValue::new(semconv::MESSAGING_BATCH_MESSAGE_COUNT, batch_input.entries().len() as i64));
                    // Extract queue name from URL
                    if let Some(queue_url) = batch_input.queue_url() {
                        if let Some(queue_name) = queue_url.split('/').last() {
                            attributes.push(KeyValue::new(semconv::MESSAGING_DESTINATION_NAME, queue_name.to_string()));
                        }
                    }
                }
            }
            "ReceiveMessage" => {
                if let Some(receive_input) = input.downcast_ref::<aws_sdk_sqs::operation::receive_message::ReceiveMessageInput>() {
                    attributes.push(KeyValue::new(semconv::MESSAGING_OPERATION_TYPE, semconv::MESSAGING_OPERATION_TYPE_RECEIVE));
                    attributes.push(KeyValue::new("server.address", receive_input.queue_url().unwrap_or_default().to_string()));
                    if let Some(max_messages) = receive_input.max_number_of_messages() {
                        attributes.push(KeyValue::new("aws.sqs.max_messages", max_messages as i64));
                    }
                    // Extract queue name from URL
                    if let Some(queue_url) = receive_input.queue_url() {
                        if let Some(queue_name) = queue_url.split('/').last() {
                            attributes.push(KeyValue::new(semconv::MESSAGING_DESTINATION_NAME, queue_name.to_string()));
                        }
                    }
                }
            }
            _ => {}
        }
        
        attributes
    }
}

/// SNS-specific attribute builder.
#[cfg(feature = "sns")]
#[cfg_attr(docsrs, doc(cfg(feature = "sns")))]
pub struct SnsAttributeBuilder;

#[cfg(feature = "sns")]
impl AttributeBuilder for SnsAttributeBuilder {
    fn build_attributes(&self, _service: &str, operation: &str, input: &dyn std::any::Any) -> Vec<KeyValue> {
        let mut attributes = vec![
            KeyValue::new(semconv::MESSAGING_SYSTEM, "aws_sns"),
        ];
        
        match operation {
            "Publish" => {
                if let Some(publish_input) = input.downcast_ref::<aws_sdk_sns::operation::publish::PublishInput>() {
                    attributes.push(KeyValue::new(semconv::MESSAGING_OPERATION_TYPE, semconv::MESSAGING_OPERATION_TYPE_SEND));
                    attributes.push(KeyValue::new("messaging.operation.name", "publish"));
                    
                    // Extract topic name from ARN
                    if let Some(topic_arn) = publish_input.topic_arn() {
                        if let Some(topic_name) = topic_arn.split(':').last() {
                            attributes.push(KeyValue::new("messaging.destination.name", topic_name.to_string()));
                        }
                    }
                    
                    // Handle target ARN for mobile push
                    if let Some(target_arn) = publish_input.target_arn() {
                        if let Some(target_name) = target_arn.split(':').last() {
                            attributes.push(KeyValue::new("messaging.destination.name", target_name.to_string()));
                        }
                    }
                }
            }
            "PublishBatch" => {
                if let Some(batch_input) = input.downcast_ref::<aws_sdk_sns::operation::publish_batch::PublishBatchInput>() {
                    attributes.push(KeyValue::new(semconv::MESSAGING_OPERATION_TYPE, semconv::MESSAGING_OPERATION_TYPE_SEND));
                    attributes.push(KeyValue::new("messaging.operation.name", "publish_batch"));
                    attributes.push(KeyValue::new(semconv::MESSAGING_BATCH_MESSAGE_COUNT, batch_input.publish_batch_request_entries().len() as i64));
                    
                    // Extract topic name from ARN
                    if let Some(topic_arn) = batch_input.topic_arn() {
                        if let Some(topic_name) = topic_arn.split(':').last() {
                            attributes.push(KeyValue::new("messaging.destination.name", topic_name.to_string()));
                        }
                    }
                }
            }
            _ => {}
        }
        
        attributes
    }
}

/// Lambda-specific attribute builder.
#[cfg(feature = "lambda")]
#[cfg_attr(docsrs, doc(cfg(feature = "lambda")))]
pub struct LambdaAttributeBuilder;

#[cfg(feature = "lambda")]
impl AttributeBuilder for LambdaAttributeBuilder {
    fn build_attributes(&self, _service: &str, operation: &str, input: &dyn std::any::Any) -> Vec<KeyValue> {
        let mut attributes = Vec::new();
        
        match operation {
            "Invoke" => {
                if let Some(invoke_input) = input.downcast_ref::<aws_sdk_lambda::operation::invoke::InvokeInput>() {
                    attributes.push(KeyValue::new("aws.lambda.function_name", invoke_input.function_name().unwrap_or_default().to_string()));
                    if let Some(invocation_type) = invoke_input.invocation_type() {
                        attributes.push(KeyValue::new("aws.lambda.invocation_type", invocation_type.as_str().to_string()));
                    }
                    if let Some(qualifier) = invoke_input.qualifier() {
                        attributes.push(KeyValue::new("aws.lambda.qualifier", qualifier.to_string()));
                    }
                }
            }
            _ => {}
        }
        
        attributes
    }
}