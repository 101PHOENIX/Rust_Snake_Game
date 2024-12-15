// grafik çizimi için gereken temel araçlar programa dahil edilir
use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;


// Game yapısında doğrudan BLOCK_SIZE'ı tutmamanın nedeni, 
// bu sabitin Game yapısının mantıksal işleyişiyle doğrudan ilgili olmamasıdır. 
//Game yapısı, oyunun kurallarını, oyunun durumunu (yılan, yem pozisyonları, oyunun bitip bitmediği vb.) 
//ve oyunun mantığını yönetir. BLOCK_SIZE ise tamamen oyunun görsel sunumuyla ilgilidir.


// bir oyun bloğunun (örneğin, yılanın bir parçası veya yemin bir karesi) 
// grafiksel olarak ne kadar genişlikte ve yükseklikte çizileceğini belirtir. 
const BLOCK_SIZE: f64 = 25.0;

pub fn game_coord_to_gui_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
} // burada BLOCK_SIZE ile çarpmak için f64 yani ondalıklı sayı formatına dönüştürdük.
// oyun alanındaki bir koordinatın, ekrandaki piksel cinsinden karşılığı bulunur.

pub fn game_coord_to_gui_coord_u32(game_coord: i32) -> u32 {
    game_coord_to_gui_coord(game_coord) as u32
} // çizim operasyonları ve piksel koordinatlarının,
// negatif değerler almaması ve her zaman pozitif veya sıfır aldığı için
// f64 türündeki GUI koordinatlarını u32 türüne dönüştürdük. 


// verilen bir renkte, belirli bir oyun koordinatındaki blok şeklini çizmek için kullanılır.
//  Piston kütüphanesinden alınan, grafik çizimi için gerekli olan bağlam (context) ve grafik çizim nesnesidir.
pub fn draw_game_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    // Bu satırlar, verilen oyun koordinatlarını GUI koordinatlarına dönüştürür.
    let gui_x = game_coord_to_gui_coord(x);
    let gui_y = game_coord_to_gui_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform, // Grafik transformasyonu (döndürme, ölçekleme, öteleme gibi) işlemleri için gerekli olan dönüşüm matrisini belirtir
        g, // Grafik çizimi için kullanılan G2d nesnesidir.
    );
}

// verilen bir renkte ve boyutta bir dikdörtgen çizmek için kullanılır.
pub fn draw_gui_rectangle(
    color: Color,
    x: i32,
    y: i32, // Çizilecek dikdörtgenin sol üst köşesinin oyun koordinatlarıdır.
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = game_coord_to_gui_coord(x);
    let gui_y = game_coord_to_gui_coord(y);

    rectangle( // Piston'daki rectangle fonksiyonu, koordinatları f64 türünde alır.
        color,
        [
            gui_x, // Dikdörtgenin sol üst köşesinin x 
            gui_y, // Dikdörtgenin sol üst köşesinin y 
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
// u32'ye koordinatları dönüştürmemiz gereken durumlar genellikle 
// grafik kartına piksel veri gönderirken karşımıza çıkar. 
// Piston gibi kütüphaneler ise çizim işlemlerini soyutlayarak geliştiricinin 
// grafik kartıyla direk etkileşim kurmasını engeller.