use std::time::Duration;
use tokio::time::sleep;
use std::time::SystemTime;

// ==========================================
// 1. OBJEK KOMPUTASI NUMERIK (Moving Average)
// ==========================================
struct MovingAverage {
    ukuran_window: usize,
    data: Vec<f64>,
}

impl MovingAverage {
    // Constructor untuk membuat objek filter baru
    fn new(ukuran: usize) -> Self {
        Self {
            ukuran_window: ukuran,
            data: Vec::new(),
        }
    }

    // Method komputasi numerik
    fn proses(&mut self, nilai_baru: f64) -> f64 {
        self.data.push(nilai_baru);

        // Jika jumlah data melebihi ukuran window, hapus data paling lama (indeks 0)
        if self.data.len() > self.ukuran_window {
            self.data.remove(0);
        }

        // Menghitung rata-rata dari seluruh isi array/vector
        let total: f64 = self.data.iter().sum();
        total / (self.data.len() as f64)
    }
}
