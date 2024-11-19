use std::{future::Future, pin::Pin, time::Duration};

use crate::CLIENT_SCRIPT_PATH;

pub(crate) async fn check_heart_beat(host: &str, timeout: Option<Duration>) -> bool {
    let timeout = match timeout {
        Some(t) => t,
        None => Duration::from_secs(10),
    };

    let ping_endpoint = match host.ends_with("/") {
        true => {
            let mut host = host.to_string();
            host.push_str(CLIENT_SCRIPT_PATH);
            host.into_boxed_str()
        }
        false => {
            let mut host = host.to_string();
            host.push('/');
            host.push_str(CLIENT_SCRIPT_PATH);
            host.into_boxed_str()
        }
    };

    let response = retry_cb(5, move || {
        let ping_endpoint = ping_endpoint.clone().to_string();
        Box::pin(async move {
            reqwest::Client::new()
                .get(ping_endpoint.as_str())
                .timeout(timeout)
                .send()
                .await
        })
    })
    .await;

    match response {
        Err(err) => {
            log::error!(
                "Failed to make HTTP request to heartbeat-check endpoint: {}",
                err
            );
            false
        }
        Ok(response) => response.status() == 200,
    }
}

async fn retry_cb<T, E>(
    mut retries_count: i32,
    cb: impl Send + Sync + 'static + Fn() -> Pin<Box<dyn Future<Output = Result<T, E>>>> + 'static,
) -> Result<T, E> {
    let mut response = cb().await;

    while response.is_err() && retries_count > 0 {
        response = cb().await;
        retries_count -= 1;
    }

    response
}

#[cfg(test)]
mod test {
    use super::retry_cb;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_retry_cb() {
        let tries_count = Arc::new(Mutex::new(0));
        let tries_clone = Arc::clone(&tries_count);

        let result = retry_cb(5, move || {
            let tries_count = tries_clone.clone();
            Box::pin(async move {
                let count = *tries_count.lock().unwrap();
                if count > 0 {
                    *tries_count.lock().unwrap() = count + 1;
                    return Err(());
                }

                Ok(())
            })
        })
        .await;

        assert!(result.is_ok());
    }
}
