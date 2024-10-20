use std::time::Duration;

use crate::CLIENT_SCRIPT_PATH;

pub(crate) async fn check_heart_beat(host: &str, timeout: Option<Duration>) -> bool {
    let timeout = match timeout {
        Some(t) => t,
        None => Duration::from_secs(10),
    };

    let ping_endpoint = match host.ends_with("/") {
        true => {
            let mut host = host.to_string();
            host.push_str(&CLIENT_SCRIPT_PATH);
            host
        },
        false => {
            let mut host = host.to_string();
            host.push('/');
            host.push_str(&CLIENT_SCRIPT_PATH);
            host
        },
    };

    let response = reqwest::Client::new()
        .get(ping_endpoint)
        .timeout(timeout)
        .send()
        .await;

    match response {
        Err(err) => {
            log::error!("Failed to make HTTP request to heartbeat-check endpoint: {}", err);
            return false;
        },
        Ok(response) => {
            return response.status() == 200;
        }
    };
}
