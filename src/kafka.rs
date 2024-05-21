use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;
use std::env;

pub fn create_producer_from_env() -> FutureProducer {
    let broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set");
    let enable_ssl = env::var("ENABLE_KAFKA_SSL").unwrap_or_else(|_| "false".to_string());

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", &broker);

    if enable_ssl.to_lowercase() == "true" {
        config.set("security.protocol", "ssl");
    }
    if let (Ok(username), Ok(password)) = (env::var("KAFKA_USERNAME"), env::var("KAFKA_PASSWORD")) {
        config
            .set("sasl.mechanism", "SCRAM-SHA-256")
            .set("security.protocol", "sasl_ssl")
            .set("sasl.username", &username)
            .set("sasl.password", &password);
    }

    config.create().expect("Producer creation error")
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_create_producer_from_env() {
        println!("hello");
    }
}