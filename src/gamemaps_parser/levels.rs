pub struct Level {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub planes: Vec<Plane>
}

pub struct Plane {
    pub data: Vec<u16>
}