pub mod member;
pub use member::MemberMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(MemberMutation);