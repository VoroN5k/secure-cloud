use common::EncryptedFile;
use std::fs;

#[tauri::command]
async fn encrypt_and_upload(file_path: String) -> Result<String, String> {
    println!("Reading File: {}", file_path);

    let data = fs::read(&file_path)
        .map_err(|e| format!("Unable to read file: {}", e))?;

    let encrypted_package = EncryptedFile {
        name_hash: format!("hash_of_{}", file_path),
        encrypted_data: data,
        nonce: vec![0u8; 12],
    };

    println!("File ready, size: {} bytes", encrypted_package.encrypted_data.len());

    Ok(format!("Файл успішно підготовлено! ({} байт)", encrypted_package.encrypted_data.len()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init()) // Додаємо плагін діалогів!
        .invoke_handler(tauri::generate_handler![encrypt_and_upload]) // Реєструємо нашу команду
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}