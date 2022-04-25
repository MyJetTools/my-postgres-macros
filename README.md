# my-postgres-macros

Macros uses tokio_postgres cargo package to generate requests;
For Date time - DateTimeAsMicroseconds is used


All the usecases are going to presented by simple Client KeyValue usecases.

Let's say we want to save a string key/value for each client

So we have a structure

```rust


pub struct KeyValueDto {
    pub client_id: String,
    pub key: String,
    pub value: String,
}
```
### Insert use-cases


```rust
#[derive(PostgresInsertModel)]
pub struct KeyValueDto {
    pub client_id: String,
    pub key: String,
    pub value: String,
}
```

this macros generates the method which we can use

```rust

let client: tokio_postgres::Client = ...

let db_model = KeyValueDto
        db_model
            .insert_db_entity(&client, TABLE_NAME)
            .await?;
            
```
