#[cfg(test)]
mod tests {
    use std::string::String;
    use tfl_api_wrapper::{Client, RequestBuilder};

    const VERSION: &str = "master.5754\r\n";

    // APP_KEY=478e5d2a343545d28ef9d3141038d603

    fn get_client(app_key: String) -> Client {
        Client::new(app_key)
    }

    #[tokio::test]
    async fn it_is_version_1() {
        let app_key = "478e5d2a343545d28ef9d3141038d603".to_string();
        let client = get_client(app_key);
        let ver = client.api_version().fetch().await.unwrap();
        assert_eq!(ver.version, VERSION, "Version is not {}", VERSION);
    }
}