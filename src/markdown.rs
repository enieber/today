use std::io::Write;
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
