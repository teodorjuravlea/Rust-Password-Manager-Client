use crate::model::{EncryptedDataEntry, EncryptedDataEntryResponse, ErrorResponse, UserResponse};
use crate::requests;

pub fn test_requests() {
    // Create a reqwest client with a cookie store
    let reqwest_client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    // Register a user
    match requests::register_request(
        "lmao@example.com",
        "password",
        "password",
        &reqwest_client,
        "http://localhost:8080/register",
    ) {
        Ok(response) => println!("Register response: {:?}", response),
        Err(e) => println!("Register failed: {}", e),
    };

    // Login
    match requests::login_request(
        "lmao@example.com",
        "password",
        &reqwest_client,
        "http://localhost:8080/login",
    ) {
        Ok(response) => println!("Login response: {:?}", response),
        Err(e) => println!("Login failed: {}", e),
    };

    // Get user info
    match reqwest_client.get("http://localhost:8080/me").send() {
        Ok(response) => println!("User info response: {:?}", response.text().unwrap()),
        Err(e) => println!("User info failed: {}", e),
    }

    // Logout
    match requests::logout_request(&reqwest_client, "http://localhost:8080/logout") {
        Ok(response) => println!("Logout response: {:?}", response),
        Err(e) => println!("Logout failed: {}", e),
    };

    // Try to get user info again
    match reqwest_client.get("http://localhost:8080/me").send() {
        Ok(response) => println!("User info response: {:?}", response.text().unwrap()),
        Err(e) => println!("User info failed: {}", e),
    };

    // Login again
    match requests::login_request(
        "lmao@example.com",
        "password",
        &reqwest_client,
        "http://localhost:8080/login",
    ) {
        Ok(response) => println!("Login response: {:?}", response.data),
        Err(e) => println!("Login failed: {}", e),
    };

    // Get user info
    match reqwest_client.get("http://localhost:8080/me").send() {
        Ok(response) => println!("User info response: {:?}", response.text().unwrap()),
        Err(e) => println!("User info failed: {}", e),
    }

    // Get all data entries
    let response = requests::get_all_encrypted_data_entries_request(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
    );

    let data_entries = response.unwrap().data;

    println!("Data entries:");
    for data_entry in data_entries {
        println!("{:?}", data_entry);
    }

    // Add a new encrypted data entry
    let data_entry = EncryptedDataEntry {
        name: "My Password".to_string(),
        content: "savedpassword1738".to_string(),
        content_type: "password".to_string(),
    };

    match requests::add_encrypted_data_entry_request(
        data_entry,
        &reqwest_client,
        "http://localhost:8080/add_encrypted_data_entry",
    ) {
        Ok(response) => println!("Add encrypted data entry response: {:?}", response),
        Err(e) => println!("Add encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    let response = requests::get_all_encrypted_data_entries_request(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
    );

    let data_entries = response.unwrap();

    println!("Data entries:");
    for data_entry in data_entries.data {
        println!("{:?}", data_entry);
    }

    // Add another encrypted data entry
    let data_entry = EncryptedDataEntry {
        name: "My Note".to_string(),
        content: "This is a note".to_string(),
        content_type: "note".to_string(),
    };

    match requests::add_encrypted_data_entry_request(
        data_entry,
        &reqwest_client,
        "http://localhost:8080/add_encrypted_data_entry",
    ) {
        Ok(response) => println!("Add encrypted data entry response: {:?}", response),
        Err(e) => println!("Add encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    let response = requests::get_all_encrypted_data_entries_request(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
    );

    let data_entries = response.unwrap();

    println!("Data entries:");
    for data_entry in data_entries.data {
        println!("{:?}", data_entry);
    }

    // Update an encrypted data entry
    match requests::update_encrypted_data_entry_request(
        "My Password",
        "My Password",
        "newpassword1738",
        "password",
        &reqwest_client,
        "http://localhost:8080/update_encrypted_data_entry",
    ) {
        Ok(response) => println!("Update encrypted data entry response: {:?}", response),
        Err(e) => println!("Update encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    let response = requests::get_all_encrypted_data_entries_request(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
    );

    let data_entries = response.unwrap();

    println!("Data entries:");
    for data_entry in data_entries.data {
        println!("{:?}", data_entry);
    }

    // Delete an encrypted data entry
    match requests::delete_encrypted_data_entry_request(
        "My Note",
        "note",
        &reqwest_client,
        "http://localhost:8080/delete_encrypted_data_entry",
    ) {
        Ok(response) => println!("Delete encrypted data entry response: {:?}", response),
        Err(e) => println!("Delete encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    let response = requests::get_all_encrypted_data_entries_request(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
    );

    let data_entries = response.unwrap();

    println!("Data entries:");
    for data_entry in data_entries.data {
        println!("{:?}", data_entry);
    }
}
