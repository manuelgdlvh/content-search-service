
use mockall::mock;
use rstest::rstest;

use lib::models::entity::Entity;
use lib::models::language::Language;
use lib::services::search_service_impl::SearchService;

mock! {
  pub SearchServiceImpl {}

  impl SearchService for SearchServiceImpl {
    fn search(&self, keywords: &mut str, lang: Language, entity: Entity) -> anyhow::Result<Vec<u64>>;
  }
}
#[tokio::test]
#[rstest]
#[case(("Language", "ES"), ("asd", "MOVIE"))]
async fn should_returns_successfully(#[case] header: (&str, &str), #[case] input: (&str, &str)) -> anyhow::Result<()> {
    /*
        // Given
    let mut header_map = HeaderMap::new();
    header_map.append(header.0, HeaderValue::from_str(header.1).unwrap());
    let search_request = SearchRequest::new(input.0.to_string(), Cow::Borrowed(input.1));

    // When
    let mut mock = State(Arc::new(MockSearchServiceImpl::new()));
    mock.expect_search()
        .times(1)
        .returning(|v0, v1, v2| {
            Ok(vec![0, 1, 2])
        });

    let body = Body::from(serde_json::to_string(&search_request)?);
    let request = Request::new(body);

    // Then
    let result = search::<MockSearchServiceImpl>(mock, header_map, JsonDeserializer::try_from(request)?).await?;
     */


    Ok(())
}