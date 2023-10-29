use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
#[graphql(description = "Ping")]
struct Ping {
    pong: String,
    pong2: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn ping() -> FieldResult<Ping> {
        Ok(Ping {
            pong: "Pong!".to_string(),
            pong2: String::from("Pong2!"),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_pong(ping1: String, ping2: String) -> FieldResult<Ping> {
        Ok(Ping {
            pong: String::from(ping1),
            pong2: String::from(ping2),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
