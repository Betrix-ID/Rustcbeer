> ![git text](/server/IFS.png)

> [!IMPORTANT]
> RustcBeer WebUI Server adalah server ringan berbasis Rust dan Actix-Web untuk menyajikan antarmuka web statis seperti  HTML, CSS, dan JavaScript secara lokal.
> Kompatibel untuk dijalankan di Android (melalui Termux) maupun sistem Linux lainnya, server ini mendeteksi folder static secara otomatis dari berbagai lokasi umum seperti /sdcard, /storage/emulated/0, dan direktori home pengguna.
> Tanpa konfigurasi tambahan, server akan langsung membuka URL di browser default saat dijalankan. Cocok digunakan untuk dokumentasi offline, UI konfigurasi lokal, atau dashboard ringan.

> [!TIP]
> Fitur Utama:
> - Deteksi otomatis direktori static
> - Kompatibel dengan Termux dan Linux
> - Berbasis Rust dan Actix-Web: cepat dan ringan
> - Penyajian langsung file HTML, CSS, JS, dan aset web lainnya
> - Membuka browser default saat server berjalan
> - Binary dapat dikompresi menggunakan UPX untuk efisiensi distribusi

> [!NOTE]
> Script ini dirancang dengan pendekatan minimalis dan fleksibel.
> Tidak memerlukan file konfigurasi atau setup tambahan.
> Cukup siapkan folder static dan jalankan binary server.
> Setelah dikompilasi, binary dapat dipindahkan dan digunakan di mana saja.

> [!WARNING]
> Pastikan struktur file sudah benar agar server dapat berjalan dengan baik:
> Folder static harus berisi file index.html
> Pastikan tidak ada kesalahan penamaan atau lokasi file
> Proses kompilasi harus berhasil tanpa error

Kompilasi WebUI

# Salin proyek ke direktori home Termux
cp -r /sdcard/RustcBeer/live ~

# Masuk ke direktori proyek
cd ~/live

# Kompilasi proyek
make

Setelah kompilasi selesai, file binary dapat dipindahkan ke lokasi lain.
Gunakan strip dan upx untuk mengurangi ukuran file jika diperlukan:

strip target/release/rustcbeer
upx --best target/release/rustcbeer

> [!INFO]
> Versi terbaru telah mendukung file konfigurasi opsional static_path.txt untuk menentukan lokasi folder static secara manual jika deteksi otomatis gagal.
> Fitur eksperimental seperti auto-reload dan autentikasi lokal sedang dalam tahap pengembangan.


<!-- Tambahkan ini di <head> HTML kamu -->
<div align="center">
  If you like my work, please follow me or star my work on GitHub
  
You can also show your concern by donating below.
<div align="center">
 </div>
<hr/>

  <div style="margin: 20px 0;">
    <a href="https://www.instagram.com/pai_calll?igsh=OGZnYmZ5OGdiMG9r" target="_blank" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/-Instagram-red?style=for-the-badge&logo=instagram&logoColor=white" alt="Instagram">
    </a>
    <a href="https://www.tiktok.com/@pai.call" target="_blank" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/-TikTok-black?style=for-the-badge&logo=tiktok&logoColor=white" alt="TikTok">
    </a>
    <a href="https://saweria.co/Uniccc" target="_blank" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/-Saweria-yellow?style=for-the-badge&logo=saweria&logoColor=white" alt="Saweria">
    </a>
    <a href="https://t.me/Yeye_PID" target="_blank" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/-Telegram-blue?style=for-the-badge&logo=telegram&logoColor=white" alt="Telegram">
    </a>
  </div>

  <hr style="border: none; height: 1px; background: #ddd; margin: 40px 0;">

</div>
