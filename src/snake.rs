// yılanın gövdesini bloklardan ve bu bloklarıda linked list içerisinde sakladım.
use std::collections::LinkedList; // Common Collections
// piston kütüphanesi ayarlanır
use piston_window::{Context, G2d};
use piston_window::types::Color;
// burada yılanın her bir bloğu ekrana çizilir.
use crate::draw::draw_game_block;

// yılanın rengi belirlenir.  
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// Copy ve Clone: Enum'un kopyalanabilir olmasını sağlar.
// PartialEq: İki yönün eşit olup olmadığını kontrol etmeye izin verir.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Bir yönü ters yöne çevirir.
   pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Yılanın gövdesindeki her bir blok için x ve y koordinatlarını tutulur.
// Debug: Hata ayıklama sırasında blokları yazdırmayı sağlar.
// Clone: Blokların kopyalanmasını mümkün kılar.
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

// Yılan tanımlanır.
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>, // Yılanın gövdesindeki bloklar linkedList'te tutulur.
    tail: Option<Block>, // Yılanın kuyruğu tanımlanır.
    should_grow_next_move: bool, // Yılanın bir sonraki hareketinde büyümesi gerekip gerekmediği kontrol edilir.
}

impl Snake {
    // yeni bir yılan oluşturulur.
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {x: x + 2, y}); // yılanın kuyruğu 
        body.push_back(Block {x: x + 1, y }); // yılanın gövdesi
        body.push_back(Block {x, y}); // yılanın başı 
        // Burada bloklar kuyruktan başa doğru sıralanır.

        Snake {
            direction: Direction::Right, // Başlangıç yönü sağa doğru olur.
            body,
            tail: None, // Yılanın kuyruğu başlangıçta None olarak ayarlanır çünkü kuyruğun silinmesi veya büyüme durumu bu aşamada gereksizdir.
            should_grow_next_move: false,
        }
    }
    // Yılanın her bir bloğunu yeşil renkte çizer.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_game_block(SNAKE_COLOR, block.x, block.y, con, g); 
            // Yılanın gövdesindekitüm blokları döngüyle dolaşır.
            // Döngü her bir blocğu için snake_color renginde çizer
        }
    }
    // Yılanın baş pozisyonu döndürülür.
    pub fn head_position(&self) -> (i32, i32) {
        self.body.front().map(|block| (block.x, block.y)).unwrap_or((0,0))
    }
    

    pub fn move_forward(&mut self, dir: Option<Direction>) { // ownership
        // yön değerine göre yeni baş pozisyon hesaplanır.
        let next_direction = dir.unwrap_or(self.direction);
        self.direction = next_direction;

        // girilen yön değerine göre kafa pozisyonu ayarlanır.
        let (head_x, head_y) = self.head_position();
        let new_head = match self.direction {
            Direction::Up => Block {x: head_x, y: head_y - 1},
            Direction::Down => Block {x: head_x, y: head_y + 1},
            Direction::Left => Block {x: head_x - 1, y: head_y},
            Direction::Right => Block {x: head_x + 1, y: head_y},};

         self.body.push_front(new_head);
        if self.should_grow_next_move {
            self.should_grow_next_move = false;
        } else {
              self.tail = self.body.pop_back();
        }
    }

    //  yılanın o anki yönünü (örneğin, Up, Down, Left, Right) saklar.
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head_position(&self, dir: Option<Direction>) -> (i32, i32) {
        // yılanın mevcut kafa pozisyonunun x ve y koordinatlarını alır ve 
        // sırasıyla head_x ve head_y değişkenlerine atar.
        let (head_x, head_y) = self.head_position();
        // dir parametresi kontrol edilir eğer değer içeriyorsa yılan o yönde haraket eder.
        // eğer içermiyorsa yılan mevcut yönünde ilerlemeye devam eder.
        let moving_direction = dir.unwrap_or(self.direction);
         match moving_direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // Yılanın bir sonraki hareketinde büyümesi gerektiğini işaretler.
    pub fn grow(&mut self) {
        self.should_grow_next_move = true;
    }
    
    // Yılanın belirli bir pozisyonda iken kendi gövdesiyle çarpışıp çarpışmadığı kontrol edilir.
    pub fn is_colliding_with_tail(&self, x: i32, y: i32) -> bool { // !!!!!!!!!!!!!!!!!
        self.body.iter().any(|block| block.x == x && block.y == y)
    } // block.x ve block.y yılan oyunu bağlamında, yılanın vücudunu oluşturan her bir blok'un konumunu temsil eden koordinatlardır.
    // x ve y bizim kontrol etmek istediğimiz kordinatlar.
    // Eğer her iki koşul da sağlandıysa (yani, verilen pozisyon, o anki iterasyon üzerindeki blok pozisyonu ile aynıysa), true döndürür.
}