use std::borrow::Cow;

use reqwest::StatusCode;
use rstest::rstest;

use lib::handlers::requests::search_request::SearchRequest;

use crate::containers::{check_post, no_output_check_post};

#[tokio::test]
#[rstest]
#[case("queen", "MOVIE", 1)]
#[case("command", "GAME", 9)]
#[case("low", "RECIPE", 3)]
#[case("split", "TV", 1)]
async fn should_returns_successfully(#[case] keywords: String, #[case] entity: &str, #[case] expected: usize) -> anyhow::Result<()> {
    let request = SearchRequest::new(keywords, Cow::from(entity));
    let response: Vec<u64> = check_post::<_, Vec<u64>>("/run", &request, StatusCode::OK).await?;
    assert_eq!(expected, response.len());
    Ok(())
}


#[tokio::test]
#[rstest]
#[case("test", "INVALID")]
async fn should_returns_bad_request(#[case] keywords: String, #[case] entity: &str) -> anyhow::Result<()> {
    let request = SearchRequest::new(keywords, Cow::from(entity));
    no_output_check_post("/run", &request, StatusCode::BAD_REQUEST).await?;
    Ok(())
}