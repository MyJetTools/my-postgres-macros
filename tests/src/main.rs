mod dto;

use std::time::Duration;

async fn main() {
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

    println!("Done");
}
