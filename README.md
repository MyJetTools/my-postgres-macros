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


### Update use-case

Primary Key - is not participated in update operations as fields to update but used to identify the record we want to update;

To mark primary keys attr #[primary_key] is used

```rust
#[derive(PostgresUpdateModel)]
pub struct KeyValueDto {
    #[primary_key]
    pub client_id: String,
    #[primary_key]
    pub key: String,
    pub value: String,
}
```

which generates the code possible to use:


```rust

let client: tokio_postgres::Client = ...

let db_model = KeyValueDto
        db_model
            .update_db_entity(&client, TABLE_NAME)
            .await?;
            
```



### Select use-case

works as simple SQL Request with Fields

#### to get single row

```rust

let client: tokio_postgres::Client = ...

let query = format!("SELECT * FROM {TABLE_NAME} WHERE client_id=$1 AND key=$2");
let result = KeyValueDto::query_single_row(
            self.client.as_ref(),
            query.as_str(),
            &[&client_id, &key],
        )
        .await?;
            
```


#### to get rows

```rust

let client: tokio_postgres::Client = ...

let query = format!("SELECT * FROM {TABLE_NAME} WHERE client_id=$1");
let result =
            KeyValueDto::query_rows(self.client.as_ref(), query.as_str(), &[&client_id])
                .await?;
            
```

