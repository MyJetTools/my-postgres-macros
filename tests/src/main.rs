mod dto;

use std::time::Duration;

#[tokio::main]
async fn main() {
    let conn_string = "host=cubi-prod.postgres.database.azure.com port=5432 dbname=keyvalue user=cubi@cubi-prod password=932qAwkqePBtyugy sslmode=require";

    let application_name = "TestApp";

    /*
    let my_postgres =
        my_postgres::MyPostgres::new(conn_string, application_name, my_logger::LOGGER.clone())
            .await;

    let result = my_postgres
        .get_count("SELECT count(*) FROM key_value".to_string(), &vec![], None)
        .await
        .unwrap();
         */

    //println!("{:?}", result);

    //  crate::bulk_insert_case::execute(&client).await;
    // crate::test_delete_case::execute(&client).await;

    //    crate::test_insert_or_update::execute(&client).await;

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Done");
}
