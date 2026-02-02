use cedar_policy::{Authorizer, Context, Decision, Entities, EntityUid, PolicySet, Request};
use std::str::FromStr;

fn main() {
    let policy_src = r#"
        permit(
            principal == User::"alice",
            action == Action::"s3:GetObject",
            resource == S3Bucket::"my-data-bucket"
        );
        permit(
            principal == User::"alice",
            action == Action::"s3:PutObject",
            resource == S3Bucket::"my-data-bucket"
        );
        permit(
            principal == User::"bob",
            action == Action::"s3:GetObject",
            resource == S3Bucket::"public-bucket"
        );
        forbid(
            principal == User::"bob",
            action == Action::"s3:DeleteObject",
            resource
        );
    "#;

    let policies = PolicySet::from_str(policy_src).expect("Failed to parse policies");
    let entities = Entities::empty();
    let authorizer = Authorizer::new();

    println!("=== Cedar Policy S3 Access Check ===\n");

    check_access(&authorizer, &policies, &entities, "alice", "s3:GetObject", "my-data-bucket");
    check_access(&authorizer, &policies, &entities, "alice", "s3:PutObject", "my-data-bucket");
    check_access(&authorizer, &policies, &entities, "alice", "s3:DeleteObject", "my-data-bucket");
    check_access(&authorizer, &policies, &entities, "bob", "s3:GetObject", "public-bucket");
    check_access(&authorizer, &policies, &entities, "bob", "s3:DeleteObject", "public-bucket");
    check_access(&authorizer, &policies, &entities, "charlie", "s3:GetObject", "my-data-bucket");
}

fn check_access(
    authorizer: &Authorizer,
    policies: &PolicySet,
    entities: &Entities,
    user: &str,
    action: &str,
    bucket: &str,
) {
    let principal = EntityUid::from_str(&format!("User::\"{}\"", user)).expect("Invalid principal");
    let action_uid = EntityUid::from_str(&format!("Action::\"{}\"", action)).expect("Invalid action");
    let resource = EntityUid::from_str(&format!("S3Bucket::\"{}\"", bucket)).expect("Invalid resource");
    let context = Context::empty();

    let request = Request::new(principal, action_uid, resource, context, None).expect("Failed to create request");
    let response = authorizer.is_authorized(&request, policies, entities);

    let decision = match response.decision() {
        Decision::Allow => "ALLOW",
        Decision::Deny => "DENY",
    };

    println!("User '{}' -> {} on '{}': {}", user, action, bucket, decision);
}
