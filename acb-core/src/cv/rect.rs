
pub struct Rectangle {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub center: (f32, f32)
}

impl Rectangle {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Rectangle {
        let center_x = left + 0.5 * width;
        let center_y = top + 0.5 * height;
        Rectangle {
            left,
            right: left + width,
            top,
            bottom: top + height,
            width,
            height,
            center_x,
            center_y,
            center: (center_x, center_y)
        }
    }
}