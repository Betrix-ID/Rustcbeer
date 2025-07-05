> [git text](/server/IFS.png)

> [!INFO]
> Script ini adalah server WebUI ringan berbasis Rust yang dirancang untuk menyajikan antarmuka web statis seperti HTML, CSS, dan JavaScript secara lokal di perangkat Android, Termux, maupun sistem Linux lainnya. Dengan mekanisme pencarian otomatis terhadap folder static yang berisi index.html, server ini memungkinkan konten web disajikan langsung dari berbagai lokasi penyimpanan seperti /sdcard, /storage/emulated/0, hingga root direktori pengguna, tanpa perlu konfigurasi manual.
> Script ini ideal digunakan untuk menyajikan antarmuka konfigurasi lokal, dokumentasi offline, atau dashboard interaktif, terutama dalam pengembangan aplikasi Android atau utilitas CLI. Server ini juga membuka URL secara otomatis di browser default saat dijalankan, menjadikannya mudah digunakan bahkan oleh pengguna non-teknis.
> Dikembangkan menggunakan bahasa pemrograman Rust dan framework Actix-Web, server ini mengutamakan efisiensi, kecepatan, dan footprint ringan. Setelah dikompilasi dalam mode release, file binari dapat dikompresi lebih lanjut menggunakan UPX agar ukuran output seminimal mungkin.
