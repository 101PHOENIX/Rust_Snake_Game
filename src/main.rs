// Grafik işleme için Piston kütüphanesini kullandım
// Rastgele sayı üretimi için Rand kütüphanesini kullandım
extern crate piston_window; 
extern crate rand;

// diğer modülleri bu modüle ekledim
mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

// game'deki game struct yapısı çağrılır.
use crate::game::Game;
// drwas'taki game_coord_to_gui_coord_u32'nu çağrılır.
use crate::draw::game_coord_to_gui_coord_u32;

// arka plan rengini ayarladım.
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
   // Oyun alanının boyutunu ayarladım
    let (width, height) = (30, 30);

   // Yeni bir pencere oluşturdum,
   // Pencerenin başlığını ve boyutlarını ayarla
   // Esc tuşuna basıldığında pencerenin kapatılmasını sağladım
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [game_coord_to_gui_coord_u32(width), game_coord_to_gui_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

   // yeni oyun nesnesi oluşturdum.
    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
       match event {
         // Her çerçeve yenilendiğinde oyunu günceller.
          Event::Loop(Loop::Update(update_args)) => {
             game.update(update_args.dt)
          }
          // Klavye girişlerine göre oyun güncellenir.
          Event::Input(Input::Button(button_args), _) => {
             if let Button::Keyboard(key) = button_args.button {
                 game.handle_key_press(key);
             }
          }
          // Oyunun mevcut durumunu çizilir
          // render_args değişkeni, Loop::Render varyantından elde edilen render argümanlarını tutmak için tanımlanmıştır.
          Event::Loop(Loop::Render(_render_args))=> {
             window.draw_2d(&event, |context, g, _| {
               clear(BACK_COLOR, g);
                game.draw(&context, g); // game'deki draw fonksiyonunu çağırır.
             });
          }
           _ => {}
        }
    }
}