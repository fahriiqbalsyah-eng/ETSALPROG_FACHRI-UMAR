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

// ==========================================
// 2. OBJEK SENSOR (Menyimpan data listrik)
// ==========================================
#[derive(Debug)]
struct SensorListrik {
    tegangan_v: f64,
    arus_a: f64,
}

impl SensorListrik {
    fn hitung_daya(&self) -> f64 {
        self.tegangan_v * self.arus_a
    }
}

// Fungsi simulasi pembacaan sensor dengan "noise" (fluktuasi palsu)
async fn baca_sensor_mentah() -> SensorListrik {
    // Trik sederhana membuat nilai tegangan dan arus naik-turun menggunakan waktu sistem
    let waktu = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    
    // Tegangan berfluktuasi antara 210.0 dan 230.5
    let tegangan_noise = if waktu % 2 == 0 { 230.5 } else { 210.0 };
    // Arus berfluktuasi antara 1.1 dan 2.5
    let arus_noise = if waktu % 3 == 0 { 2.5 } else { 1.1 };

    SensorListrik {
        tegangan_v: tegangan_noise,
        arus_a: arus_noise,
    }
}

// ==========================================
// 3. MAIN PROGRAM (Sistem Monitoring & Kontrol)
// ==========================================
#[tokio::main]
async fn main() {
    println!("Memulai Sistem Monitoring Energi Listrik...");
    println!("Mengaktifkan Filter Moving Average...");
    println!("===============================================");

    // Membuat objek filter: Kita akan mengambil rata-rata dari 5 data terakhir
    let mut filter_tegangan = MovingAverage::new(5);
    let mut filter_arus = MovingAverage::new(5);

    loop {
        // A. Akuisisi Data Sensor (Noisy/Kasar)
        let data_mentah = baca_sensor_mentah().await;

        // B. Komputasi Numerik (Penyaringan/Filtering)
        let tegangan_halus = filter_tegangan.proses(data_mentah.tegangan_v);
        let arus_halus = filter_arus.proses(data_mentah.arus_a);

        // C. Membuat Objek Sensor dari data yang sudah dihaluskan
        let data_bersih = SensorListrik {
            tegangan_v: tegangan_halus,
            arus_a: arus_halus,
        };

        let daya_aktif = data_bersih.hitung_daya();

        // D. Logika Keputusan dan Alarm (Percabangan)
        let mut status = "NORMAL";
        if daya_aktif > 450.0 {
            status = "⚠️ ALARM: OVERLOAD DAYA!";
        } else if daya_aktif < 250.0 {
            status = "⬇️ WARNING: DAYA RENDAH";
        }

        // E. Output Monitoring ke Terminal
        println!(
            "Tegangan: {:>6.2} V | Arus: {:>4.2} A | Daya: {:>6.2} W | Status: {}",
            data_bersih.tegangan_v, data_bersih.arus_a, daya_aktif, status
        );

        // Jeda 1 detik sebelum siklus selanjutnya
        sleep(Duration::from_secs(1)).await;
    }
}
