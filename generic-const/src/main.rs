#[derive(Debug)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }
    
    fn set(&mut self, row: usize, col: usize, value: T) {
        if row < ROWS && col < COLS {
            self.data[row][col] = value;
        }
    }
}

fn main() {
    let mut matrix: Matrix<i32, 2, 3> = Matrix::new();
    matrix.set(0, 1, 42);
    println!("Matrix: {:?}", matrix);
}