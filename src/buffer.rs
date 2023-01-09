/// Buffer struct for storing image color data
pub struct Buffer {
    e: [i32],
    width: i32,
    height: i32,
}

impl Buffer {
    pub fn new(width: i32, height: i32) -> Buffer {
        Buffer {
            e: [i32; width * height],
            width,
            height,
        }
    }

    /// Returns the 1d array index of [r(ow), c(olumn)]
    fn index(&self, r: i32, c: i32) -> i32 {
        return r * height + c;
    }

    /// Change value at [x, y] to value
    pub fn write(&mut self, value: i32, x: i32, y: i32) {
        self.e[index(y, x)] = value;
    }

    /// Returns value at [x, y]
    pub fn at(&self, x: i32, y: i32) -> i32 {
        self.e[index(y, x)]
    }
}
