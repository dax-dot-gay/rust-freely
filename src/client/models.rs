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
        use super::users::User;
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub access_token: String,
            pub user: User
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