use std::sync::Arc;
use testcontainers::{clients, Container, Image};
use std::time::Duration;
use lazy_static::lazy_static;
use testcontainers::clients::Cli;
use testcontainers::core::{Port, WaitFor};
use testcontainers::images::kafka::Kafka;

pub(crate) mod graph_tests;
pub(crate) mod kafka_testcontainers;
pub(crate) mod kafka_metric_data_test_system;
pub(crate) mod mock_metric_data_test_system;

