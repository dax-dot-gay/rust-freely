pub mod api_models {
    pub mod users {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct User {
            pub username: String,
            pub email: Option<String>,
            pub created: Option<String>
        }

        
    }

    pub mod responses {
        use std::fmt::Debug;

        use super::users::User;
        use serde_derive::{Deserialize, Serialize};
        use serde_json::Value;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub access_token: String,
            pub user: User
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct ResponseModel {
            pub code: u16,
            pub data: Value
        }
    }

    pub mod requests {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub alias: String,
            pub pass: String
        }
    }
}