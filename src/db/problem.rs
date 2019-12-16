
#[derive(Insertable)]
[table_name="problems"]
struct InsertableProblem {
    data: serde_json::value::Value
}

#[derive(Queryable, Serializable)]
struct Problem {
    id: i32,
    data: serde_json::value::Value
}