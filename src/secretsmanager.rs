//use rusoto_core::Region;
//use rusoto_secretsmanager::{SecretsManagerClient as Client};
//
//use config::Config;
//
//pub struct SecretsManagerClient {
//    inner: Client,
//}
//
//impl Default for SecretsManagerClient {
//    fn default() -> Self {
//        SecretsManagerClient::new(region)
//    }
//}
//
//impl SecretsManagerClient {
//    pub fn new(region: Region) -> Self {
//        SecretsManagerClient { inner: Client::simple(region) }
//    }
//}
