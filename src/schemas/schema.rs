use juniper::{EmptySubscription, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
#[graphql(description = "Ping")]
struct Ping {
    pong: String,
    pong2: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A member of a team")]
struct Member {
    id: i32,
    name: String,
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
    fn member() -> FieldResult<Member> {
        Ok(Member {
            id: 1,
            name: String::from("value"),
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
    fn create_memberschema(name: String) -> FieldResult<Member> {
        Ok(Member { id: 2, name: name })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
