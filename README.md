> ![git text](/server/IFS.png)

# RustcBeer WebUI Server

Server WebUI ringan berbasis Rust dan Actix-Web, dirancang untuk menyajikan halaman web statis secara lokal di perangkat Android (Termux) maupun Linux. Cocok untuk dokumentasi offline, UI konfigurasi, atau dashboard lokal.

------

> [!IMPORTANT]
> RustcBeer WebUI Server mendeteksi folder `static` secara otomatis dari berbagai lokasi seperti:
> - `/sdcard`
> - `/storage/emulated/0`
> - Direktori root pengguna
> 
> Saat dijalankan, server akan langsung membuka URL lokal di browser default pengguna. Tidak diperlukan konfigurasi tambahan.

> [!TIP]
> Fitur Unggulan:
> - Deteksi otomatis direktori `static`
> - Kompatibel dengan Android (Termux) dan Linux
> - Penyajian HTML, CSS, JS, dan aset web lainnya
> - Dibangun dengan Rust dan Actix-Web: cepat dan ringan
> - Browser default terbuka otomatis saat server berjalan
> - Binary dapat dikompresi dengan UPX untuk distribusi

------

> [!NOTE]
> Versi terbaru mendukung file konfigurasi opsional `static_path.txt` untuk menentukan lokasi folder `static` secara manual apabila deteksi otomatis gagal.
> 
> Fitur eksperimental seperti *auto-reload* dan *autentikasi lokal* sedang dalam tahap pengembangan.

------

> [!WARNING]
> Pastikan struktur proyek sudah benar agar server dapat berjalan dengan baik:
> - Folder `static` harus berisi `index.html`
> - Tidak ada kesalahan nama file atau lokasi
> - Proses kompilasi berhasil tanpa error

------

## Ingin belajar cara memodifikasi dan mengompilasi ini?

```bash
# 1. Modifikasi folder and files)
  Download sumber Porjek (RustcBeer).
# 2.)
  Modifikasi atau tambahakan files projek
  (Html ,css or javascript) kamu ke dalam
  folder static.
# 3.)
  Pastikan semua dir yang ada di dalam makefile
  and main.rs sudah anda edit sesuai dir projek kalian

# 4. Pengcompilasian projek di dalam termux)
  Pindahkan folder projek yang kalian sudah edit ke
  Home dir nya termux dengan cara
  cp -r /dir jalur /dir name projek ~

# 5.)
  Setelah itu kalian bisa install semua
  tools² nya terlebih dahulu

  pkg update -y && pkg upgrade && pkg install rust -y
  pkg install make -y && pkg install upx -y

# 6.)
  kalian bisa masuk ke dalam folder yang di dalam nya
  ada file (makefile), setelah itu Jalankan kompilasi
   [ make ]

````
> ![big text](/server/scp.png)

>  hasil akhir dari compiler projek.
>  Semoga informasi ini dapat menbatu dan menambah ilmu anda...
  

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
