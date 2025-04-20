use sha2::{Digest, Sha256};
use tracing::info;
use std::io::{Write};
use today::{generate_name_file_today, open_file, create_file};


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

pub fn add_task(content: String) -> Result<String, String> {
    let file_path = generate_name_file_today();
    let id_task = generate_id_task(content.clone());
    let task = format!("- [ ] {} - {}", id_task, content);
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

pub fn un_done_task(id_task: String) -> Result<String, String> {
    let file_path = generate_name_file_today();
    let old_file = change_task(id_task, "".to_string());
    let file = create_file(&file_path);
    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", old_file.join("\n"));
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

pub fn done_task(id_task: String) -> Result<String, String> {
    let file_path = generate_name_file_today();
    let old_file = change_task(id_task, "x".to_string());
    let file = create_file(&file_path);
    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", old_file.join("\n"));
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

fn change_task(id_task: String, new_state: String) -> Vec<String> {
    let file_path = generate_name_file_today();
    let mut result = Vec::new();

    let match_id: String = if new_state.contains("x") {
        format!("- [ ] {}", id_task)
    } else {
        format!("- [x] {}", id_task)
    };

    let done_id: String = if new_state.contains("x") {
        format!("- [x] {}", id_task)
    } else {
        format!("- [ ] {}", id_task)
    };

    for line in std::fs::read_to_string(file_path).unwrap().lines() {
        if line.contains(&match_id) {
            info!("match_id: {}", match_id);
            info!("done_id: {}", done_id);
            let new_line = str::replace(line, &match_id, &done_id);
            info!("new_line line: {}", new_line);
            result.push(new_line)
        } else {
            result.push(line.to_string())
        }
    }

    result
}
