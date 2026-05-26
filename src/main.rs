// src/main.rs
use log::{debug, error, info, trace, warn};
use std::thread;
use std::time::Duration;

// ============================================
// MODULE 1: BUS REGULER
// ============================================
mod bus_reguler {
    use log::{error, info, warn};

    pub fn jalan() {
        info!("🚌 Bus Reguler #12 mulai berangkat");

        // Simulasi masalah
        warn!("⚠️ Bus Reguler #12: BBM mulai menipis (20% tersisa)");

        // Simulasi error
        if true {
            error!("❌ Bus Reguler #12: Mesin overheat di KM 45!");
        }

        info!("✅ Bus Reguler #12 selesai perjalanan");
    }
}

// ============================================
// MODULE 2: LOGISTIK
// ============================================
mod logistik {
    use log::{error, info, warn};

    pub fn kirim_barang() {
        info!("🚚 Logistik: Mengirim barang ke gudang pusat");

        warn!("⚠️ Logistik: Barang hampir kadaluarsa (5 hari lagi)");

        // Simulasi error serius
        if true {
            error!("❌ Logistik: Gagal mengirim barang! Truk mogok");
        }
    }
}

// ============================================
// MODULE 3: TIKET
// ============================================
mod tiket {
    use log::{debug, info, trace, warn};

    pub fn jual() {
        info!("🧾 Tiket: Sistem penjualan tiket dimulai");

        debug!("🔍 Tiket: User Ucup login dengan ID 12345");
        trace!("📝 Tiket: Detail transaksi - kode=TK001, harga=50000");

        warn!("⚠️ Tiket: Stok tiket tinggal 10 lembar");

        info!("✅ Tiket: Penjualan selesai, total 50 tiket terjual");
    }
}

// ============================================
// MODULE 4: MAINTENANCE (Bisa diatur sendiri)
// ============================================
mod maintenance {
    use log::{debug, info, trace};

    pub fn cek_mesin() {
        info!("🔧 Maintenance: Pengecekan rutin semua bus");

        debug!("🔍 Maintenance: Cek mesin Bus #12 - OK");
        debug!("🔍 Maintenance: Cek rem Bus #13 - OK");
        debug!("🔍 Maintenance: Cek lampu Bus #14 - OK");

        trace!("📝 Maintenance: Detail tekanan ban = 35 PSI");
        trace!("📝 Maintenance: Detail oli mesin = 98%");

        info!("✅ Maintenance: Semua bus siap beroperasi");
    }
}

// ============================================
// MAIN FUNCTION
// ============================================
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inisialisasi logger dari file konfigurasi
    log4rs::init_file("log4rs.yaml", Default::default())?;

    info!("🚀 TERMINAL BUS DIMULAI!");

    // Jalankan semua module
    bus_reguler::jalan();
    logistik::kirim_barang();
    tiket::jual();
    maintenance::cek_mesin();

    info!("🏁 TERMINAL BUS AKAN TUTUP dalam 10 detik...");

    // Tunggu 10 detik biar Promtail sempat baca log
    thread::sleep(Duration::from_secs(10));

    info!("👋 TERMINAL BUS TUTUP! Sampai jumpa!");

    Ok(())
}
