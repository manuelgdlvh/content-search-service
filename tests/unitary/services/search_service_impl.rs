use mockall::{mock, predicate};
use rstest::rstest;

use lib::infrastructure::di_container::{DIContainer, GAME_INDEX_PROCESSOR_DEP, MOVIE_INDEX_PROCESSOR_DEP, RECIPE_INDEX_PROCESSOR_DEP, TV_INDEX_PROCESSOR_DEP};
use lib::models::entity::Entity;
use lib::models::language::Language;
use lib::services::index_processor::{IndexSearcher, IndexWriter};
use lib::services::search_service_impl::{SearchService, SearchServiceImpl};

mock! {
  pub IndexProcessor {}
  impl IndexSearcher for IndexProcessor {
    fn search<'a>(&self, lang: Language, tokens: &'a [&'a str]) -> anyhow::Result<Vec<u64>>;
    }
}
#[tokio::test]
#[rstest]
#[case(("KEYWORDS".to_string(), Language::Es, Entity::Movie), vec ! [0, 1, 2])]
async fn should_returns_successfully(#[case] mut input: (String, Language, Entity), #[case] expected: Vec<u64>) -> anyhow::Result<()> {

    // Given/When
    let expected_result = expected.clone();
    let keywords_lowercase = input.0.to_lowercase();
    let tokens = keywords_lowercase.split_whitespace().collect::<Vec<&str>>();

    let di_container = DIContainer::new();
    let mut mock_movie_index_processor = MockIndexProcessor::new();
    mock_movie_index_processor.expect_search().with(predicate::eq(input.1), predicate::eq(tokens)).returning(move |_, _| Ok(expected_result.clone()));

    di_container.add(MOVIE_INDEX_PROCESSOR_DEP, mock_movie_index_processor);
    di_container.add(TV_INDEX_PROCESSOR_DEP, MockIndexProcessor::new());
    di_container.add(RECIPE_INDEX_PROCESSOR_DEP, MockIndexProcessor::new());
    di_container.add(GAME_INDEX_PROCESSOR_DEP, MockIndexProcessor::new());

    // Then
    let service = SearchServiceImpl::<MockIndexProcessor>::new(&di_container);
    let result = service.search(input.0.as_mut(), input.1, input.2)?;

    assert_eq!(expected, result);
    Ok(())
}