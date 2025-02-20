mod hardware;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Monitor de Hardware em Tempo Real para macOS");
    println!("Pressione Ctrl+C para sair");

    // Loop principal para atualização em tempo real
    loop {
        // Limpa a tela (simulação de atualização)
        print!("\x1B[2J\x1B[1;1H");

        // Coleta de informações do sistema
        let cpu_usage = hardware::get_cpu_usage();
        let memory_usage = hardware::get_memory_usage();

        // Exibe as informações
        println!("Uso de CPU: {}%", cpu_usage.unwrap_or("Erro".to_string()));
        println!(
            "Uso de Memória: {} MB",
            memory_usage.unwrap_or("Erro".to_string())
        );
        // Atualiza a cada 2 segundos
        thread::sleep(Duration::from_secs(2));
    }
}