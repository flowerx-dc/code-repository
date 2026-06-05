use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== ROZPOCZĘCIE SKRYPTU INSTALACYJNEGO ===\n");

    // 1. Sprawdzamy pierwszą komendę (np. amixer)
    let program = "amixer";
    println!("[*] Sprawdzanie czy narzędzie '{}' jest dostępne...", program);

    if uruchom_z_animacja("which", &[program]) {
        println!("[+] Sukces: Narzędzie '{}' jest już zainstalowane!", program);
    } else {
        println!("[-] Błąd: Nie znaleziono narzędzia '{}'.", program);
        
        // 2. Interakcja z użytkownikiem w razie niepowodzenia
        print!("[?] Czy chcesz spróbować zainstalować 'alsa-utils' zamiast tego? (y/n): ");
        io::stdout().flush().unwrap(); // Wymuszenie wyświetlenia tekstu przed pytaniem

        let mut odpowiedz = String::new();
        io::stdin().read_line(&mut odpowiedz).expect("Nie udało się odczytać linii");

        if odpowiedz.trim().to_lowercase() == "y" {
            println!("[*] Instalowanie alsa-utils (może wymagać sudo)...");
            // Przykładowa komenda instalacji (używamy apt)
            if uruchom_z_animacja("sudo", &["apt", "install", "-y", "alsa-utils"]) {
                println!("[+] Instalacja zakończona sukcesem!");
            } else {
                println!("[-] Instalacja nie powiodła się.");
            }
        } else {
            println!("[i] Pominięto instalację alternatywną.");
        }
    }

    println!("\n=== KONIEC SKRYPTU ===");
}

/// Funkcja uruchamia komendę i wyświetla animację ładowania (- \ | /)
fn uruchom_z_animacja(komenda: &str, argumenty: &[&str]) -> bool {
    // Kanał do komunikacji między wątkiem roboczym a wątkiem animacji
    let (tx, rx) = mpsc::channel();
    
    // Przygotowanie komendy do uruchomienia
    let cmd_string = komenda.to_string();
    let args_vec: Vec<String> = argumenty.iter().map(|s| s.to_string()).collect();

    // Wątek roboczy: wykonuje komendę w tle
    thread::spawn(move || {
        let status = Command::new(cmd_string)
            .args(&args_vec)
            .stdout(Stdio::null()) // Ukrywamy standardowe wyjście komendy, żeby nie psuło animacji
            .stderr(Stdio::null()) // Ukrywamy błędy komendy
            .status();

        // Wysyłamy informację zwrotną: true jeśli komenda się udała (status 0)
        let sukces = match status {
            Ok(s) => s.success(),
            Err(_) => false,
        };
        let _ = tx.send(sukces);
    });

    // Wątek główny: wyświetla kręcącą się animację
    let klatki_animacji = ['-', '\\', '|', '/'];
    let mut i = 0;

    loop {
        // Sprawdzamy, czy wątek roboczy już skończył pracę
        if let Ok(wynik) = rx.try_recv() {
            // Czyszczenie znaku animacji na koniec
            print!("\r");
            io::stdout().flush().unwrap();
            return wynik;
        }

        // Wyświetlanie kolejnej klatki animacji (\r cofa kursor na początek linii)
        print!("\r[{}] Ładowanie... ", klatki_animacji[i]);
        io::stdout().flush().unwrap();

        i = (i + 1) % klatki_animacji.len();
        thread::sleep(Duration::from_millis(100)); // Prędkość obracania się animacji
    }
}
