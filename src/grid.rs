use rand::Rng;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

pub struct Grid {
    pub size_x: i32,
    pub size_y: i32,
}

impl Grid {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        return Grid { size_x, size_y };
    }

    pub fn create_position(x: i32, y: i32) -> GridPosition {
        GridPosition { x, y }
    }

    pub fn random_position(&self) -> GridPosition {
        let mut rng = rand::thread_rng();
        Self::create_position(
            rng.gen_range::<i32, i32, i32>(0, self.size_x),
            rng.gen_range::<i32, i32, i32>(0, self.size_y),
        )
    }

    fn is_in_range(&self, pos: &GridPosition) -> bool {
        pos.x >= 0 && pos.x < self.size_x &&pos.y >= 0 && pos.y < self.size_y
    }

    pub fn for_each_neighbour<F>(&self, pos: GridPosition, mut f: F) 
    where
    F: FnMut(&GridPosition)
     {
        {
            let p = Self::create_position(pos.x, pos.y - 1);
            if self.is_in_range(&p) {
                f(&p);
            }
        }
        {
            let p = Self::create_position(pos.x, pos.y + 1);
            if self.is_in_range(&p) {
                f(&p);
            }
        }
        {
            let p = Self::create_position(pos.x - 1, pos.y);
            if self.is_in_range(&p) {
                f(&p);
            }
        }
        {
            let p = Self::create_position(pos.x + 1, pos.y);
            if self.is_in_range(&p) {
                f(&p);
            }
        }
    }

}


trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where
    T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        // Because of our trait bounds, we can now apply these operators.
        (self.clone() % n.clone() + n.clone()) % n.clone()
    }
}
