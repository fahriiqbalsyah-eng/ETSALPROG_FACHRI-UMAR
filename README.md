# ⚡ Sistem Monitoring Energi Listrik Berbasis Rust

Repositori ini berisi source code untuk Project Based Learning (PBL) Evaluasi Tengah Semester (ETS) mata kuliah **Algoritma dan Pemrograman**.

## 📖 Deskripsi Proyek
Program ini adalah simulasi sistem instrumentasi cerdas untuk memonitor parameter kelistrikan (Tegangan dan Arus) secara *real-time*. Sistem ini dilengkapi dengan komputasi numerik **Simple Moving Average (SMA)** untuk menyaring *noise* data sensor, serta logika kontrol presisi untuk mendeteksi anomali daya.

## ✨ Fitur Utama
- **Pembacaan Sensor Asinkron:** Menggunakan library `tokio` untuk simulasi pembacaan data secara *real-time* tanpa memblokir sistem.
- **Filter Numerik:** Implementasi *Moving Average* untuk menghaluskan fluktuasi data transien.
- **Sistem Proteksi (Relay Logic):**
  - 🚨 **ALARM OVERLOAD:** Aktif jika daya melebihi 450 Watt (Relay OFF).
  - ⬇️ **WARNING DAYA RENDAH:** Aktif jika daya di bawah 250 Watt.
  - ✅ **NORMAL:** Beroperasi di rentang aman 250W - 450W (Relay ON).
- **Validasi Data:** Mencegah kalkulasi dari data sensor yang tidak rasional (misal: tegangan minus).

## 🛠️ Cara Menjalankan Program
Pastikan Anda sudah menginstal [Rust](https://www.rust-lang.org/tools/install) di komputer Anda. Buka terminal, masuk ke folder repositori ini, lalu ketik perintah berikut:

> cargo run

## 👥 Tim Pengembang
- **Muhammad Fachri Iqbal Syahputra** (NRP: 2042251129)
- **Umar Fathin Mufid** (NRP: 2042251118)

**Departemen Teknik Instrumentasi | Institut Teknologi Sepuluh Nopember (ITS) Surabaya**
