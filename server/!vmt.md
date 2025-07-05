# Date : 05 - 07 - 2025          
           [ RustcBeer - Api Webui ]
                   Author 
     Telegram @UnixeID | Github Betrix-ID
                version : 1.0

Script ini adalah server WebUI ringan berbasis Rust yang dirancang untuk menyajikan antarmuka web statis seperti HTML, CSS, dan JavaScript secara lokal di perangkat Android, Termux, maupun sistem Linux lainnya. Dengan mekanisme pencarian otomatis terhadap folder static yang berisi index.html, server ini memungkinkan konten web disajikan langsung dari berbagai lokasi penyimpanan seperti /sdcard, /storage/emulated/0, hingga root direktori pengguna, tanpa perlu konfigurasi manual.

Script ini ideal digunakan untuk menyajikan antarmuka konfigurasi lokal, dokumentasi offline, atau dashboard interaktif, terutama dalam pengembangan aplikasi Android atau utilitas CLI. Server ini juga membuka URL secara otomatis di browser default saat dijalankan, menjadikannya mudah digunakan bahkan oleh pengguna non-teknis.

Dikembangkan menggunakan bahasa pemrograman Rust dan framework Actix-Web, server ini mengutamakan efisiensi, kecepatan, dan footprint ringan. Setelah dikompilasi dalam mode release, file binari dapat dikompresi lebih lanjut menggunakan UPX agar ukuran output seminimal mungkin.

Fitur Utama:
 - Deteksi otomatis direktori static di berbagai lokasi sistem file.
 - Kompatibel dengan perangkat Android (via Termux) maupun Linux reguler.
 - Berbasis Rust & Actix-Web: ringan, cepat, dan stabil.
 - Menyajikan file statis dengan index.html sebagai entry point.
 - Mendukung JavaScript, CSS, dan aset statis lainnya.
 - Otomatis membuka browser default saat server aktif.
 - Binari dapat dikompresi menggunakan UPX untuk efisiensi distribusi.

Dengan desain yang fleksibel dan minimalis, script ini dapat digunakan sebagai solusi webserver lokal untuk keperluan pribadi, debugging, hingga pengujian frontend secara cepat dan praktis.

Try Compiler Script server:
 1. edit and move files (html, css and js) to static folder  
 2. move all your projects to termux server then compile with `make` 
 3. After the compiler is finished, you can move the compiled project to wherever you want.  
 Last step
 Make sure all compiler outputs are free from errors or mistakes (file names, wrong directories, etc.) because if you compile it incorrectly, the server will not work.  
 
 Hopefully this explanation can help you all 
 
# Commnad Compiler :
- cp -r /sdcard/name folder ~
- cd /name foler 
- make
 
# Command running shell
sh /sdcard/RustcBeer/run.sh

# ⚠️ Especially for those who want to use my script in your Module, please contact me & add credit @UnixeID