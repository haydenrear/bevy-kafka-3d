use serde::Deserialize;

#[derive(Deserialize)]
pub struct KafkaConfiguration {
    pub(crate) hosts: Vec<String>,
    pub(crate) consumer_group_id: String,
    pub(crate) client_id: String
}

impl Default for KafkaConfiguration {
    fn default() -> Self {
        Self {
            hosts: vec!["localhost:9092".to_string()],
            consumer_group_id: "consumer".to_string(),
            client_id: "nn-fe".to_string()
        }
    }
}
