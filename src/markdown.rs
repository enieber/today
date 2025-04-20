use std::io::{Write};
use sha2::{Sha256, Digest};
use today::{create_file, open_file, generate_name_file_today, date_string};

pub fn new_markdown(content: String) -> Result<String, String> {
    let file_path = generate_name_file_today();
    let today_str = date_string();
    let into = format!(r#"# Today - {}

{}

"#, today_str, content);
    let file = create_file(&file_path);
    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", &into);
            match has_write {
                Ok(_ok) => {
                    return Ok(format!("Line write with success"));
                }
                Err(err) => {
                    return Err(format!("Error to write file: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error to open file: {}", err));
        }
    }
}

pub fn update_markdown(content: String) -> Result<String, String> {
    let file_path = generate_name_file_today();
    let file = open_file(&file_path);
    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", &content);
            match has_write {
                Ok(_ok) => {
                    return Ok(format!("Line write with success"));
                }
                Err(err) => {
                    return Err(format!("Error to write file: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error to open file: {}", err));
        }
    }
}

fn generate_id_task(task: String) -> String {
    let header = format!("blob {}\0", task.len());
    let data = [header.as_bytes(), task.as_bytes()].concat();

    // Cálculo da hash SHA-256
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // Conversão para hexadecimal
    let full_hash: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();
    full_hash.chars().take(7).collect()
}

pub fn add_task_markdown(content: String) -> Result<String, String> {
    let file_path = generate_name_file_today();
    let id_task = generate_id_task(content.clone());
    let task = format!("- [] {} - {}", id_task, content);
    let file = open_file(&file_path);
    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", &task);
            match has_write {
                Ok(_ok) => {
                    return Ok(format!("Line write with success"));
                }
                Err(err) => {
                    return Err(format!("Error to write file: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error to open file: {}", err));
        }
    }
}
