pub mod api_wrapper {
    use crate::api_client::{self, Client};

    #[derive(Clone, Debug)]
    pub struct Api {
        client: Client
    }

    impl Api {
        pub fn new(client: Client) -> Self {
            Api {client}
        }

        pub fn base(&self) -> String {
            self.client.url()
        }

        pub fn token(&self) -> Option<String> {
            self.client.token()
        }

        pub fn is_authenticated(&self) -> bool {
            self.client.is_authenticated()
        }
    }
}