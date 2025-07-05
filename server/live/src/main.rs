/*!
 * RustcBeer WebUI Server API
 * Copyright (C) 2025 @UnixeID
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use actix_files::Files;
use actix_web::{App, HttpServer};
use std::{net::TcpListener, path::PathBuf, process::Command};

/// Ganti path ini sesuai keinginan user (bebas rename atau pindah folder)
const STATIC_PATH: &str = "/storage/emulated/0/RustcBeer/live/static";

/// Cek apakah folder static valid (ada index.html)
fn validate_static_dir(path: &str) -> Option<PathBuf> {
    let path_buf = PathBuf::from(path);
    if path_buf.join("index.html").exists() {
        Some(path_buf)
    } else {
        eprintln!("❌ index.html tidak ditemukan di: {}", path);
        None
    }
}

/// Membuka browser di Android
#[cfg(target_os = "android")]
fn open_browser_fallback(url: &str) {
    let _ = Command::new("am")
        .args(["start", "-a", "android.intent.action.VIEW", "-d", url])
        .status();
}

/// Membuka browser di selain Android
#[cfg(not(target_os = "android"))]
fn open_browser_fallback(url: &str) {
    let _ = webbrowser::open(url);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let static_dir = validate_static_dir(STATIC_PATH)
        .expect("⚠️ Folder static tidak valid! Pastikan index.html tersedia.");

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let url = format!("http://{}", addr);

    println!("✅ Server berjalan di: {}", url);
    println!("📂 Menyajikan konten dari: {}", STATIC_PATH);

    open_browser_fallback(&url);

    HttpServer::new(move || {
        App::new().service(Files::new("/", &static_dir).index_file("index.html"))
    })
    .listen(listener)?
    .run()
    .await
}