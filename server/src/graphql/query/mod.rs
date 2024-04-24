pub mod member;
pub use member::MemberQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(MemberQuery);
