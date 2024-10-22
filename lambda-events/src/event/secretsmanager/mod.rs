use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecretsManagerSecretRotationEvent {
    pub step: String,
    pub secret_id: String,
    pub client_request_token: String,
}

#[cfg(test)]
mod test {
    // To save on boiler plate, JSON data is parsed from a mut byte slice rather than an &str. The slice isn't actually mutated
    // when using serde-json, but it IS when using simd-json - so we also take care not to reuse the slice
    // once it has been deserialized.

    use super::*;

    #[test]
    #[cfg(feature = "secretsmanager")]
    fn example_secretsmanager_secret_rotation_event() {
        let mut data = include_bytes!("../../fixtures/example-secretsmanager-secret-rotation-event.json").to_vec();
        let parsed: SecretsManagerSecretRotationEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: SecretsManagerSecretRotationEvent =
            aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
