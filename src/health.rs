use std::{fmt::write, future::Future, pin::Pin};

async fn is_website_alive_before_refactoring() -> Result<(), String> {
    let response = reqwest::get("http://example.com")
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    if text.contains("illustrative") {
        Ok(())
    } else {
        Err("text missing".into())
    }
}

trait NetworkAdapter {
    async fn fetch_url_text(&self, url: String) -> Result<String, String>;
}

async fn is_website_alive<N>(network_adaptor: N) -> Result<(), String>
where
    N: NetworkAdapter,
{
    let text = network_adaptor
        .fetch_url_text("http://example.com".into())
        .await
        .map_err(process_fetch_error)?;

    process_fetch_content(text)
}

async fn is_website_alive_2<F, Fut>(url_text_fetcher: F) -> Result<(), String>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = Result<String, String>>,
{
    let text = url_text_fetcher("http://example.com".into())
        .await
        .map_err(process_fetch_error)?;

    process_fetch_content(text)
}

fn process_fetch_content(text: String) -> Result<(), String> {
    if text.contains("illustrative") {
        Ok(())
    } else {
        Err("missing text".into())
    }
}

fn process_fetch_error(text: String) -> String {
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    mockall::mock! {
        NetworkAdapter {}

        impl NetworkAdapter for NetworkAdapter {
            async fn fetch_url_text(&self, url: String) -> Result<String, String>;
        }
    }

    #[tokio::test]
    async fn website_health_check_returns_ok() {
        let result = is_website_alive_before_refactoring().await;

        assert!(result.is_ok());
    }

    #[test]
    fn website_up_check_on_fetch_success_with_good_content_returns_true() {
        let response = process_fetch_content("illustrative".into());

        assert!(response.is_ok());
    }

    #[test]
    fn website_up_check_on_fetch_fail_throws_error() {
        let response = process_fetch_error("error text".into());

        assert_eq!(response, "error text");
    }

    #[tokio::test]
    async fn website_verifier_with_good_content_returns_true() {
        let mut stub_network_adaptor = MockNetworkAdapter::new();
        stub_network_adaptor
            .expect_fetch_url_text()
            .return_const(Ok("illustrative".into()));

        let response = is_website_alive(stub_network_adaptor).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn website_verifier_with_bad_content_returns_false() {
        let mut stub_network_adaptor = MockNetworkAdapter::new();
        stub_network_adaptor
            .expect_fetch_url_text()
            .return_const(Ok("hello".into()));

        let response = is_website_alive(stub_network_adaptor).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), "missing text");
    }

    #[tokio::test]
    async fn website_verifier_2_with_good_content_returns_true() {
        let stub = |_url| async move { Ok("illustrative".into()) };
        let response = is_website_alive_2(stub).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn website_verifier_2_with_bad_content_returns_false() {
        let stub = |_url| async move { Ok("hello".into()) };
        let response = is_website_alive_2(stub).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), "missing text");
    }
}
