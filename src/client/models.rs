pub mod api_models {
    pub mod users {
        use serde_derive::{Deserialize, Serialize};
        use chrono::{DateTime, Utc};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct User {
            pub username: String,
            pub email: Option<String>,
            pub created: Option<DateTime<Utc>>
        }

        
    }

    pub mod posts {
        use serde_derive::{Deserialize, Serialize};
        use chrono::{DateTime, Utc};

        use crate::api_client::Client;

        use super::collections::Collection;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum PostAppearance {
            #[serde(rename = "sans")]
            SansSerif,

            #[serde(rename = "serif")]
            #[serde(alias = "norm")]
            #[serde(alias = "serif")]
            Serif,

            #[serde(rename = "wrap")]
            Wrap,

            #[serde(rename = "mono")]
            Mono,

            #[serde(rename = "code")]
            Code
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Post {
            pub client: Option<Client>,
            pub id: String,
            pub slug: Option<String>,
            pub appearance: Option<PostAppearance>,
            pub language: Option<String>,
            pub rtl: bool,
            pub created: Option<DateTime<Utc>>,
            pub title: Option<String>,
            pub body: String,
            pub tags: Vec<String>,
            pub views: Option<u64>,
            pub collection: Option<Collection>
        }

        impl Post {
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct PostCreation {
            pub body: String,
            pub title: Option<String>,
            pub font: Option<PostAppearance>,
            pub lang: Option<String>,
            pub rtl: Option<bool>,
            pub created: Option<DateTime<Utc>>
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

    pub mod collections {
        use serde_derive::{Deserialize, Serialize};

        use crate::api_client::Client;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Collection {
            pub client: Option<Client>,
            pub alias: String,
            pub title: String,
            pub description: Option<String>,
            pub style_sheet: Option<String>,
            pub public: bool,
            pub views: Option<u64>,
            pub verification_link: Option<String>,
            pub total_posts: Option<u64>,
        }

        impl Collection {
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }
        }
    }
}