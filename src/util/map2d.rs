use super::Vec2;

pub struct Map2d<Tile> {
    pub size: Vec2,
    pub data: Vec<Tile>,
}

impl<Tile> Map2d<Tile> {
    pub fn new_default(size: Vec2, default: Tile) -> Self
    where
        Tile: Clone,
    {
        let data = vec![default; (size.x * size.y) as usize];
        Self { size, data }
    }

    pub fn parse_grid(s: &str, f: impl Fn(char) -> Tile) -> Self {
        let size_x = s.lines().next().unwrap().len();
        let size_y = s.lines().count();
        let size = Vec2::new(size_x as i64, size_y as i64);

        let data = s
            .chars()
            .filter(|&c| c != '\n')
            .map(f)
            .collect::<Vec<_>>();
        
        Self { size, data }
    }
    
    pub fn index_of(&self, pos: Vec2) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            None
        } else {
            Some((pos.x + pos.y * self.size.x) as usize)
        }
    }
    
    pub fn pos_of(&self, index: usize) -> Vec2 {
        let x = index as i64 % self.size.x;
        let y = index as i64 / self.size.x;
        Vec2::new(x, y)
    }

    pub fn get(&self, pos: Vec2) -> Option<Tile> where Tile: Copy {
        self.index_of(pos).map(|i| self.data[i])
    }
    
    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut Tile> {
        self.index_of(pos).map(move |i| &mut self.data[i])
    }
    
    pub fn get_row(&self, y: i64) -> &[Tile] {
        let start = self.index_of(Vec2::new(0, y)).unwrap();
        let end = self.index_of(Vec2::new(self.size.x - 1, y)).unwrap();
        &self.data[start..=end]
    }
    
    pub fn find(&self, predicate: impl Fn(&Tile) -> bool) -> Option<Vec2> {
        self.data.iter().position(predicate).map(|i| self.pos_of(i))
    }
}