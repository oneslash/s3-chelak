use chrono::{DateTime, Utc};
use std::time::SystemTime;

/// Get timestamp in RFC3339 format
/// Coverts quickly from SystemTime to RFC3339 format
pub fn get_timestamp(sys_time: SystemTime) -> String {
    let datetime = DateTime::<Utc>::from(sys_time);
    datetime.to_rfc3339()
}

/// Validate location constraint
/// https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html#API_CreateBucket_RequestSyntax
pub fn validate_location_constraint(location_constraint: &str) -> bool {
    let locations: Vec<&str> = vec![
        "us-east-1",
        "us-east-2",
        "us-west-1",
        "us-west-2",
        "af-south-1",
        "ap-east-1",
        "ap-south-1",
        "ap-northeast-3",
        "ap-northeast-2",
        "ap-southeast-1",
        "ap-southeast-2",
        "ap-northeast-1",
        "ca-central-1",
        "cn-north-1",
        "cn-northwest-1",
        "eu-central-1",
        "eu-west-1",
        "eu-west-2",
        "eu-south-1",
        "eu-west-3",
        "eu-north-1",
        "me-south-1",
        "sa-east-1",
        "us-gov-east-1",
        "us-gov-west-1",
    ];
    
    locations.contains(&location_constraint)
}
