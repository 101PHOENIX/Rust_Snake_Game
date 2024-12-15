use piston_window::types::Color;
use piston_window::*; // Grafik çizimi için Piston kütüphanesi eklenir
// Rastgele sayılar üretmek için Rand kütüphanesi eklenir
use rand::{thread_rng, Rng};

// Çizim işlevleri eklenir
use crate::draw::{draw_game_block, draw_gui_rectangle};



// yılan modülündeki tanımlanmış öğeler bu dosyaya aktarılır.
use crate::snake::{Direction, Snake};


// Yemeğin, sınırların ve oyun bittiğindeki renklerin durumu ayarlanır.
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

// Yılanın hareket aralığını ve oyunun başlangıç süresi ayarlanır
const MOVE_PERIOD: f64 = 0.1;
const RESTART_DELAY: f64 = 1.0;

pub struct Game { // Common Collections
    snake: Snake,
    food: Option<(i32, i32)>, // yemeğin kordinatı tutulur
    width: i32,
    height: i32,
    game_over: bool, // oyunun bitip bitmediği kontrol edilir
    time_since_last_move: f64, //  Yılanın son hareket zamanını kontrol eder.
    time_since_game_over: f64, //  Oyun bittikten sonra geçen zamanı kontrol eder.
}


impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let mut game =  Game {
            snake: Snake::new(2, 2), // yılan 2. satır ve 2. sütunda başlar
            food: Some((6, 4)), // ilk yemeği ise 6.satır ve 4. sütunda yerleştirilir
            width, // oyunun genişliği
            height, // oyunun yüksekliği
            game_over: false, // oyunun başlangıçta bitmediği kontrol edilir
            time_since_last_move: 0.0, 
            time_since_game_over: 0.0, 
        };

       game.add_food_if_needed(); // Eğer yiyecek yoksa bir yiyecek oluşturur
        game
    }

    // Klavye girdisine göre yılanın yönünü güncellenir
    pub fn handle_key_press(&mut self, key: Key) {
        // oyunun bitip bitmediği kontrol edilir
        if self.game_over {
            return;
        }

        // Yön tuşlarına göre yeni yön belirlenir
        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // 
        if let Some(new_dir) = direction {
                // Basılan tuştaki yönün geçerli olup olmadığı kontrol edilir.
                // Böylece yılanın kendi ters yönüne gitmesi engellenir.
                if new_dir == self.snake.head_direction().opposite() {
                   return;
                }
                // Eğer yeni yön geçerli bir yönse yılanın  yönü güncellenir.
                self.update_snake(Some(new_dir));
            } else {
                self.update_snake(None);
            }

    }
    // yılan çizilir
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);
        
        // yemeği kırmızı blok olarak çizilir
        if let Some((food_x, food_y)) = self.food {
            draw_game_block(FOOD_COLOR, food_x, food_y, con, g);
        }

        // oyun alanının sınırları çizilir
        draw_gui_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_gui_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_gui_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_gui_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // oyun bittiğinde yılan kendisine veya sınıra çarptığında tüm alan kırmızıya kaplanır.
        if self.game_over {
           draw_gui_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }
     // oyunun zamanı güncellenir

    // Yılanın hareket zamanını güncellenir.

    pub fn update(&mut self, delta_time: f64) {
        // oyunun bitip bitmediği kontrol edilir.
        // oyun bitmişse oyunun bitişinden sonra,
        // oyunun bittiğinin anlaşılması için bir bekleme süresi  1 saniye eklenir.
        if self.game_over {
            // Oyun bittiğinden bu yana geçen süreyi tutar. 
            self.time_since_game_over += delta_time;
            // time_since_game_over
            // RESTART_DELAY -> oyunun yeniden başlatılması için gereken minimum bekleme süresinden
            // fazla ise oyun yeniden başlatılır.
           if self.time_since_game_over > RESTART_DELAY {
                self.restart();
           }
           return;
        }

        self.time_since_last_move += delta_time;
        
        // Eğer self.food None ise, yeni bir yiyecek ekler.
        self.add_food_if_needed();
        // yılanın son hareketinden bu yana geçen süre 
        // haraket süresinden fazla ise yılan haraket eder.
        // Kısaca; Eğer yeterli zaman geçmişse yılan haraket eder.
        if self.time_since_last_move > MOVE_PERIOD {
            self.update_snake(None);
            self.time_since_last_move = 0.0;
        }
        
    }

    // Eğer yemek yoksa bir yemek oluşturur.
    fn add_food_if_needed(&mut self) {
         if self.food.is_none() { // Common Collections
             self.add_food();
         }
    }
    // Yılanın kafasının yemekle çarpışıp çarpışmadığını kontrol edilir.
    // Eğer çarpışma varsa veya aynı konumdalarsa yemek kaldırılır ve yılanın büyümesi sağlanır.
    fn check_eating(&mut self) {
       if let Some((food_x, food_y)) = self.food {
           let (head_x, head_y) = self.snake.head_position();
                if food_x == head_x && food_y == head_y {
                    self.food = None;
                    self.snake.grow();
            }
        }

    }
    // Yılanın kendine çarpıp çarpmadığını veya sınırların dışına çıkıp çıkmadığını kontrol eder.
    fn is_snake_alive(&self, next_move: Option<Direction>) -> bool {
        // Yılanın bir sonraki adımda başının olacağı koordinatlar alınır.
        let (next_x, next_y) = self.snake.next_head_position(next_move);

        // verilen koordinattta yılanın gövdesindeki herhangi bir blokla çakışıp çakışmadığını kontrol eddilir.
        // eğer çakışma var ise false döner.
        if self.snake.is_colliding_with_tail(next_x, next_y) {
             return false;
         }
        
         // next_x = sol sınırı, sol sınırın dışına çıkmamış olmalı.
         // next_y = üst sınırı, üst sınırın dışına çıkmamış olmalı.
         // next_x < self.width - 1 yılanın başı, sağ sınırın dışına çıkmamış olmalı
         // next_y < self.height - 1 Yılanın başı, alt sınırın dışına çıkmamış olmalı.
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
    // Rastgele bir koordinatta yemek oluşturur ve bu noktanın yılanın üzerine düşmediği kontrol edilir.
    fn add_food(&mut self) {
        // rastgele bir sayı üretilir.
        let mut rng = thread_rng();
        let mut new_x;
        let mut new_y;
        loop {
            // rastgele bir x kordinatı üretilir
            // new_x değeri 1 ile self.width - 1 (en sağdaki nokta) arasında olur
            // new_y değeri 1 ile self.width - 1 (en alttaki nokta) arasında olur
            // sınır bloklarına denk gelmez
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);

            // new_x ve new_y koordinatlarının yılanın gövdesindeki herhangi bir blokla çakışıp çakışmadığını kontrol edilir.
             if !self.snake.is_colliding_with_tail(new_x, new_y) {
                break;
             }
        } // eğer çakışma var ise yeni bir kordinat üretilir.
        self.food = Some((new_x, new_y));
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        // yılanın hayatta olup olmdaığı kontrol edilir.
        // eğer hayatta ise yılan bir sonraki konuma taşınır
        // yemeği yiyip yemediği kontrol edilir.
        if self.is_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            // Eğer ölmüş ise game_over = true yapılır.
            // Bir restert_delay eklemek için time_since_game_over sıfır yapılır.
            // sıfırlanmazsa, oyun bir sonraki güncelleme döngüsünde hemen yeniden başlar.
           self.game_over = true;
           self.time_since_game_over = 0.0;
       }
    }

    // Oyun başlangıçtaki ayarına geri çekilir.
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food = Some((6,4));
        self.game_over = false;
        self.time_since_last_move = 0.0;
        self.time_since_game_over = 0.0;
    }
}