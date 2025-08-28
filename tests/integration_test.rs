use mp::{AppConfig, MicroblogService, PostStatus};

#[test]
fn test_library_imports() {
    // Test that we can create the types that should be exported
    let service = MicroblogService::new(
        "https://example.com/micropub".to_string(),
        "test-token".to_string()
    );
    
    let config = AppConfig::new(service);
    assert_eq!(config.service.api_url, "https://example.com/micropub");
    assert_eq!(config.service.auth_token, "test-token");
    
    // Test that PostStatus can be created and used
    let status = PostStatus::Published;
    assert_eq!(status.as_str(), "published");
    
    let draft_status = PostStatus::Draft;
    assert_eq!(draft_status.as_str(), "draft");
}

#[test]
fn test_microblog_service_from_args() {
    let result = MicroblogService::from_args(
        "https://test.com/micropub".to_string(),
        "secret-token".to_string()
    );
    
    assert!(result.is_ok());
    let service = result.unwrap();
    assert_eq!(service.api_url, "https://test.com/micropub");
    assert_eq!(service.auth_token, "secret-token");
}

#[test]
fn test_microblog_service_from_args_empty() {
    let result = MicroblogService::from_args(
        "".to_string(),
        "token".to_string()
    );
    
    assert!(result.is_err());
    
    let result2 = MicroblogService::from_args(
        "https://test.com".to_string(),
        "".to_string()
    );
    
    assert!(result2.is_err());
}
