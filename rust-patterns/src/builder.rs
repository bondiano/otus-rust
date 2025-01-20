#[derive(Debug)]
pub struct MySuperSdk {
    pub url: String,
    pub user_data: String,
}

impl MySuperSdk {
    pub fn builder() -> MySuperSdkBuilder {
        MySuperSdkBuilder {
            url: None,
            user_data: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct MySuperSdkBuilder {
    url: Option<String>,
    user_data: Option<String>,
}

impl MySuperSdkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(self, url: String) -> Self {
        Self {
            url: Some(url),
            ..self
        }
    }

    pub fn user_data(self, user_data: String) -> Self {
        Self {
            user_data: Some(user_data),
            ..self
        }
    }

    pub fn build(self) -> Result<MySuperSdk, String> {
        Ok(MySuperSdk {
            url: self.url.ok_or_else(|| "No Url Provided".to_string())?,
            user_data: self
                .user_data
                .ok_or_else(|| "No User Data Provided".to_string())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let sdk = MySuperSdk::builder()
            .url("https://example.com".to_string())
            .user_data("user_data".to_string())
            .build()
            .unwrap();
        assert_eq!(sdk.url, "https://example.com");
        assert_eq!(sdk.user_data, "user_data");
    }
}
