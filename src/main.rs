use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use chrono::Utc;
use rand::Rng;
use std::collections::HashMap;

const AUTHOR: &str = "Rafal Oleszczak";
const PORT: u16 = 8080;

const LOCATIONS: &[(&str, &str)] = &[
    ("Polska", "Warszawa"),
    ("Polska", "Kraków"),
    ("Polska", "Gdańsk"),
    ("Niemcy", "Berlin"),
    ("Niemcy", "Monachium"),
    ("Francja", "Paryż"),
    ("Francja", "Lyon"),
    ("Włochy", "Rzym"),
    ("Włochy", "Mediolan"),
    ("Hiszpania", "Madryt"),
    ("Hiszpania", "Barcelona"),
    ("USA", "Nowy Jork"),
    ("USA", "Los Angeles"),
    ("Japonia", "Tokio"),
    ("Wielka Brytania", "Londyn"),
];

async fn home() -> impl Responder {
    let mut html = String::from(r#"
        <!DOCTYPE html>
        <html lang="pl">
        <head>
            <meta charset="UTF-8">
            <title>Wybierz lokalizację</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    background: #f0f2f5;
                    padding: 2rem;
                    color: #333;
                }
                h1 {
                    color: #007acc;
                }
                form {
                    margin-top: 1rem;
                }
                select, button {
                    padding: 0.5rem;
                    font-size: 1rem;
                    margin-right: 0.5rem;
                }
                .footer {
                    margin-top: 2rem;
                    font-size: 0.9rem;
                    color: #666;
                }
            </style>
        </head>
        <body>
            <h1>Wybierz lokalizację</h1>
            <form method='GET' action='/weather'>
                <label for='city'>Miasto:</label>
                <select name='city' id='city'>
    "#);

    for &(_, city) in LOCATIONS {
        html.push_str(&format!("<option value='{}'>{}</option>", city, city));
    }

    html.push_str(r#"
                </select>
                <button type='submit'>Pokaż pogodę</button>
            </form>
            <div class="footer">Autor: Rafal Oleszczak</div>
        </body>
        </html>
    "#);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn weather(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let city = query.get("city").cloned().unwrap_or_else(|| "Warszawa".to_string());

    let mut rng = rand::thread_rng();
    let temp = rng.gen_range(-10..=35);
    let fake_temp = format!("{}°C", temp);

    let html = format!(r#"
        <!DOCTYPE html>
        <html lang="pl">
        <head>
            <meta charset="UTF-8">
            <title>Pogoda</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    background: #fdfdfd;
                    padding: 2rem;
                    color: #222;
                }}
                h1 {{
                    color: #007acc;
                }}
                a {{
                    display: inline-block;
                    margin-top: 1rem;
                    text-decoration: none;
                    color: #007acc;
                }}
            </style>
        </head>
        <body>
            <h1>Aktualna pogoda w {}: {}</h1>
            <a href="/">← Wróć</a>
        </body>
        </html>
    "#, city, fake_temp);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let now = Utc::now();
    println!(
        "[START] Data: {}, Autor: {}, Port: {}",
        now.to_rfc3339(),
        AUTHOR,
        PORT
    );

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/weather", web::get().to(weather))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
