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
