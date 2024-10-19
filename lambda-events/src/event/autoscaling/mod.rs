use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use aws_lambda_json_impl::Value;
use std::collections::HashMap;

use crate::custom_serde::deserialize_lambda_map;

/// `AutoScalingEvent` struct is used to parse the json for auto scaling event types //
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoScalingEvent<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    /// The version of event data
    #[serde(default)]
    pub version: Option<String>,
    /// The unique ID of the event
    #[serde(default)]
    pub id: Option<String>,
    /// Details about event type
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Source of the event
    #[serde(default)]
    pub source: Option<String>,
    /// AccountId
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Event timestamp
    pub time: DateTime<Utc>,
    /// Region of event
    #[serde(default)]
    pub region: Option<String>,
    /// Information about resources impacted by event
    pub resources: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    pub detail: HashMap<String, T1>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "autoscaling")]
    fn example_autoscaling_event_launch_successful() {
        let mut data = include_bytes!("../../fixtures/example-autoscaling-event-launch-successful.json").to_vec();
        let parsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "autoscaling")]
    fn example_autoscaling_event_launch_unsuccessful() {
        let mut data = include_bytes!("../../fixtures/example-autoscaling-event-launch-unsuccessful.json").to_vec();
        let parsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "autoscaling")]
    fn example_autoscaling_event_lifecycle_action() {
        let mut data = include_bytes!("../../fixtures/example-autoscaling-event-lifecycle-action.json").to_vec();
        let parsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "autoscaling")]
    fn example_autoscaling_event_terminate_action() {
        let mut data = include_bytes!("../../fixtures/example-autoscaling-event-terminate-action.json").to_vec();
        let parsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "autoscaling")]
    fn example_autoscaling_event_terminate_successful() {
        let mut data = include_bytes!("../../fixtures/example-autoscaling-event-terminate-successful.json").to_vec();
        let parsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "autoscaling")]
    fn example_autoscaling_event_terminate_unsuccessful() {
        let mut data = include_bytes!("../../fixtures/example-autoscaling-event-terminate-unsuccessful.json").to_vec();
        let parsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(data.as_mut_slice()).unwrap();
        let mut output = aws_lambda_json_impl::to_string(&parsed).unwrap().into_bytes();
        let reparsed: AutoScalingEvent = aws_lambda_json_impl::from_slice(output.as_mut_slice()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
