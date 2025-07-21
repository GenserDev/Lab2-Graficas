use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Duration;
use gif::{Frame, Encoder, Repeat};
use std::fs::File;

// ===== CONFIGURACIÓN =====
const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 8;
const WINDOW_WIDTH: usize = WIDTH * SCALE;
const WINDOW_HEIGHT: usize = HEIGHT * SCALE;
const MAX_FRAMES: u32 = 200;
const FPS: u64 = 10; // Frames por segundo

// ===== TIPOS =====
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

// ===== ESTRUCTURA PRINCIPAL =====
pub struct GameOfLife {
    grid: Vec<Vec<CellState>>,
    width: usize,
    height: usize,
}

// ===== IMPLEMENTACIÓN DEL JUEGO =====
impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![CellState::Dead; width]; height],
            width,
            height,
        }
    }

    ///Inicializa el grid con patrones aleatorios y conocidos
    pub fn initialize(&mut self) {
        self.clear_grid();
        self.add_random_cells(0.15); // 15% de probabilidad inicial
        self.add_known_patterns();
    }

    ///Limpia todo el grid
    fn clear_grid(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                *cell = CellState::Dead;
            }
        }
    }

    ///Células aleatorias al grid
    fn add_random_cells(&mut self, probability: f64) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if rng.gen_bool(probability) {
                    self.grid[y][x] = CellState::Alive;
                }
            }
        }
    }

    ///Patrones conocidos
    fn add_known_patterns(&mut self) {
        self.add_glider(15, 15);
        self.add_glider(70, 10);
        self.add_block(5, 5);
        self.add_block(90, 90);
        self.add_blinker(25, 25);
        self.add_toad(35, 35);
        self.add_beacon(45, 45);
        self.add_beehive(60, 60);        
        self.add_lightweight_spaceship(10, 50);
        self.add_pulsar(50, 20);
    }

    /// Avanza una generación aplicando las reglas de Conway
    pub fn next_generation(&mut self) {
        let mut new_grid = self.grid.clone();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_live_neighbors(x, y);
                new_grid[y][x] = self.apply_rules(self.grid[y][x], neighbors);
            }
        }
        
        self.grid = new_grid;
    }

    /// Aplica las reglas de Conway a una célula
    fn apply_rules(&self, current_state: CellState, neighbors: usize) -> CellState {
        match (current_state, neighbors) {
            // Una célula viva con menos de 2 vecinos muere (soledad)
            (CellState::Alive, n) if n < 2 => CellState::Dead,
            // Una célula viva con 2 o 3 vecinos sobrevive
            (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
            // Una célula viva con más de 3 vecinos muere (sobrepoblación)
            (CellState::Alive, n) if n > 3 => CellState::Dead,
            // Una célula muerta con exactamente 3 vecinos nace
            (CellState::Dead, 3) => CellState::Alive,
            // Cualquier otro caso mantiene el estado
            (state, _) => state,
        }
    }

    /// Cuenta los vecinos vivos de una célula
    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if self.is_valid_position(nx, ny) && self.is_alive(nx as usize, ny as usize) {
                    count += 1;
                }
            }
        }
        
        count
    }

    /// Verifica si una posición es válida dentro del grid
    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    /// Verifica si una célula está viva
    fn is_alive(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == CellState::Alive
    }

    /// Obtiene el color de una célula para renderizado
    pub fn get_color(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            match self.grid[y][x] {
                CellState::Alive => 0x00FFFFFF, // Blanco
                CellState::Dead => 0x00001122,  // Azul oscuro
            }
        } else {
            0x00000000 // Negro para posiciones inválidas
        }
    }

    /// Renderiza el juego en un buffer de píxeles
    pub fn render(&self, buffer: &mut Vec<u32>) {
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                let grid_x = x / SCALE;
                let grid_y = y / SCALE;
                let color = self.get_color(grid_x, grid_y);
                buffer[y * WINDOW_WIDTH + x] = color;
            }
        }
    }

    /// Convierte el grid actual a datos para frame del GIF
    pub fn to_gif_frame_data(&self) -> Vec<u8> {
        let mut frame_data = vec![0; WIDTH * HEIGHT];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = y * WIDTH + x;
                frame_data[index] = match self.grid[y][x] {
                    CellState::Alive => 1, // Índice del color blanco en la paleta
                    CellState::Dead => 0,  // Índice del color azul en la paleta
                };
            }
        }
        frame_data
    }
}

// ===== PATRONES CONOCIDOS =====
impl GameOfLife {
    /// Agrega un Glider (se mueve diagonalmente)
    fn add_glider(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega un Block (estructura estática)
    fn add_block(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (0, 1), (1, 0), (1, 1)];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega un Blinker (oscilador período 2)
    fn add_blinker(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (2, 0)];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega un Toad (oscilador período 2)
    fn add_toad(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega un Beacon (oscilador período 2)
    fn add_beacon(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (0, 1), (1, 0), (2, 3), (3, 2), (3, 3)];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega un Beehive (estructura estática)
    fn add_beehive(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega una Lightweight Spaceship (nave espacial)
    fn add_lightweight_spaceship(&mut self, x: usize, y: usize) {
        let pattern = [
            (0, 0), (3, 0),
            (4, 1),
            (0, 2), (4, 2),
            (1, 3), (2, 3), (3, 3), (4, 3)
        ];
        self.add_pattern(x, y, &pattern);
    }

    /// Agrega un Pulsar (oscilador período 3)
    fn add_pulsar(&mut self, x: usize, y: usize) {
        let pattern = [
            // Cruz superior
            (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
            (0, 2), (5, 2), (7, 2), (12, 2),
            (0, 3), (5, 3), (7, 3), (12, 3),
            (0, 4), (5, 4), (7, 4), (12, 4),
            (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
            // Cruz inferior (espejo)
            (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
            (0, 8), (5, 8), (7, 8), (12, 8),
            (0, 9), (5, 9), (7, 9), (12, 9),
            (0, 10), (5, 10), (7, 10), (12, 10),
            (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
        ];
        self.add_pattern(x, y, &pattern);
    }

    /// Helper para añadir un patrón dado como coordenadas relativas
    fn add_pattern(&mut self, base_x: usize, base_y: usize, pattern: &[(usize, usize)]) {
        for &(dx, dy) in pattern {
            let x = base_x + dx;
            let y = base_y + dy;
            if x < self.width && y < self.height {
                self.grid[y][x] = CellState::Alive;
            }
        }
    }
}

// ===== GENERADOR DE GIF =====
pub struct GifGenerator {
    encoder: Encoder<File>,
}

impl GifGenerator {
    pub fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::create(filename)?;
        let color_map = [
            0x00, 0x11, 0x22, // Azul oscuro
            0xFF, 0xFF, 0xFF  // Blanco 
        ];
        
        let mut encoder = Encoder::new(file, WIDTH as u16, HEIGHT as u16, &color_map)?;
        encoder.write_extension(gif::ExtensionData::Repetitions(Repeat::Infinite))?;
        
        Ok(Self { encoder })
    }

    pub fn add_frame(&mut self, frame_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut frame = Frame::default();
        frame.width = WIDTH as u16;
        frame.height = HEIGHT as u16;
        frame.buffer = std::borrow::Cow::Borrowed(frame_data);
        frame.delay = (100 / FPS) as u16; // Convertir FPS a centisegundos
        
        self.encoder.write_frame(&frame)?;
        Ok(())
    }
}

// ===== FUNCIÓN PRINCIPAL =====
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando Conway's Game of Life...");
    
    // Inicializar juego
    let mut game = GameOfLife::new(WIDTH, HEIGHT);
    game.initialize();
    
    // Configurar GIF
    let mut gif_generator = GifGenerator::new("conway_game_of_life.gif")?;
    
    // Configurar ventana
    let mut window = Window::new(
        "Conway's Game of Life - Presiona ESC para salir",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )?;
    
    window.limit_update_rate(Some(Duration::from_millis(1000 / FPS)));
    
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut generation = 0;
    
    println!("Generando {} frames del juego...", MAX_FRAMES);
    
    while window.is_open() 
        && !window.is_key_down(Key::Escape) 
        && generation < MAX_FRAMES 
    {
        // Actualizar simulación
        game.next_generation();
        generation += 1;
        
        // Renderizar en ventana
        game.render(&mut buffer);
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;
        
        // Añadir frame al GIF
        let frame_data = game.to_gif_frame_data();
        gif_generator.add_frame(&frame_data)?;
        
        // Mostrar progreso cada 20 generaciones
        if generation % 20 == 0 {
            println!("Generación {}/{}", generation, MAX_FRAMES);
        }
    }
    
    Ok(())
}