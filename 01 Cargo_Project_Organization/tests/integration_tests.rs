use cargo_project_organization::{add, math, Config, User};

#[test]
fn public_add_is_available_to_integration_tests() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(math::double_for_public_api(5), 10);
}

#[test]
fn stats_average_handles_empty_slice() {
    assert_eq!(math::stats::average(&[]), None);
}

#[test]
fn config_and_user_are_public_api() {
    let config = Config::new("production", false);
    let user = User::new(1, "Alice");

    assert!(config.is_production());
    assert_eq!(user.display_name(), "#1 Alice");
}
