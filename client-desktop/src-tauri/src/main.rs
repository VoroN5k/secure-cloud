// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::EncryptedFile;
use std::fs;

#[tauri::command]
async fn encrypt_and_upload(file_path: String) -> Result<String, String> {
    println!("Reading File: {}", file_path);

    // 1. Read bytes of file
    let data = fs::read(&file_path)
        .map_err(|e| format!("Unable to read encrypted file: {}", e))?;

    // 2. Test encryption (for now, just wrap in struct)
    let encrypted_package = EncryptedFile {
        name_hash: format!("hash_of_{}", file_path),
        encrypted_data: data,
        nonce: vec![0u8; 12], // Empty for test
    };

    // 3. Request to Axum server
    println!("File encrypted, size: {} bytes", encrypted_package.encrypted_data.len());

    Ok(format!("Файл успішно підготовлено! ({} байт)", encrypted_package.encrypted_data.len()))
}

fn main() {
    client_desktop_lib::run();
}
