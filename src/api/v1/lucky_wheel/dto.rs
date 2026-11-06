use serde::{Deserialize, Serialize};

pub mod api_request {
    use super::*;

    #[derive(Deserialize)]
    pub struct SignIn {
        #[serde[default]]
        pub username: String,
        #[serde[default]]
        pub password: String,
    }

    #[derive(Deserialize)]
    pub struct WheelItemAdd {
        pub name: String,
        pub weight: u16,
    }

    #[derive(Deserialize)]
    pub struct WheelItemUpdate {
        #[serde(rename = "_id")]
        pub id: String,
        pub name: Option<String>,
        pub weight: Option<u16>,
    }

    #[derive(Deserialize)]
    pub struct VoucherCreate {
        pub user: String,
    }

    #[derive(Deserialize)]
    pub struct Spin {
        pub code: String,
    }
}

pub mod api_response {
    use super::*;

    #[derive(Serialize)]
    pub struct Base<T> {
        pub success: bool,
        pub message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<T>,
    }
    impl<T> Base<T> {
        pub fn success(message: impl Into<String>, _data: T) -> Self {
            Self {
                success: true,
                message: message.into(),
                data: Some(_data),
            }
        }
    }
    impl Base<()> {
        pub fn failed(message: impl Into<String>) -> Self {
            Self {
                success: false,
                message: message.into(),
                data: None,
            }
        }
    }

    #[derive(Serialize)]
    pub struct SignInData {
        pub session_token: String,
    }
}
