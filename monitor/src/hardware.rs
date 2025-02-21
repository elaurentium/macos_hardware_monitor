use std::io;
use std::process::Command;

pub fn get_cpu_usage() -> Result<String, io::Error> {
    let output = Command::new("top")
        .args(&["-l", "1", "-n", "0"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("CPU usage") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                return Ok(parts[2].to_string()); // Retorna o valor de uso da CPU do usuário
            }
        }
    }
    Ok("Desconhecido".to_string())
}

// Função para obter o uso da memória (via comando 'vm_stat')
pub fn get_memory_usage() -> Result<String, io::Error> {
    let output = Command::new("vm_stat")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut total_used_mb = 0.0;

    for line in stdout.lines() {
        if line.contains("Pages active") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Ok(pages) = parts[2].trim_end_matches('.').parse::<f64>() {
                total_used_mb += pages * 4096.0 / 1024.0 / 1024.0; // Converte páginas para MB
            }
        }
    }
    Ok(format!("{:.2}", total_used_mb))
}