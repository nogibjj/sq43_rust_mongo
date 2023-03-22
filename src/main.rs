use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://localhost:27017";
    let mut client_options =
        ClientOptions::parse(uri)
            .await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Get the 'customers' collection from the 'test' database:
    let customers = client.database("test").collection("customers");
    // Delete all documents for customers called "test":
    let delete_result = customers.delete_many(
        doc! {
            "name": "updated",
        },
        None,
    ).await?;
    println!("Deleted {} documents", delete_result.deleted_count);

    let new_cus = doc! {
        "name": "test",
        "_id": "123"
    };
    let insert_result = customers.insert_one(new_cus.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);
    // Look up one document:
    let customer = customers
        .find_one(
            doc! {
                "name": "test"
            },
            None,
        ).await?
        .expect("Missing 'Customers' document.");
    println!("Movie: {}", customer);

    let update_result = customers
    .update_one(
        doc! {
            "_id": &insert_result.inserted_id,
        },
        doc! {
            "$set": { "name": "updated" }
        },
        None,
    )
    .await?;
    println!("Updated {} documents", update_result.modified_count);

        // Look up the document again to confirm it's been updated:
        let cus = customers
        .find_one(
            doc! {
                "_id": &insert_result.inserted_id,
            },
            None,
        )
        .await?
        .expect("Missing 'Parasite' document.");
    println!("Updated Movie: {}", &cus);
    Ok(())
}