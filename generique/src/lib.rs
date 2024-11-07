pub trait Parallelogramme {
    fn largeur(&self) -> i32;
    fn hauteur(&self) -> i32;
    fn angle(&self) -> i32;
}

pub struct Carre {
    pub largeur: i32,
}
impl Parallelogramme for Carre {
    fn largeur(&self) -> i32 {
        self.largeur
    }
    fn hauteur(&self) -> i32 {
        self.largeur
    }
    fn angle(&self) -> i32 {
        90
    }
}

pub struct Rectangle {
    pub largeur: i32,
    pub hauteur: i32,
}
impl Parallelogramme for Rectangle {
    fn largeur(&self) -> i32 {
        self.largeur
    }
    fn hauteur(&self) -> i32 {
        self.hauteur
    }
    fn angle(&self) -> i32 {
        90
    }
}

pub struct ParallelogrammeGeneral {
    pub largeur: i32,
    pub hauteur: i32,
    pub angle: i32,
}
impl Parallelogramme for ParallelogrammeGeneral {
    fn largeur(&self) -> i32 {
        self.largeur
    }
    fn hauteur(&self) -> i32 {
        self.hauteur
    }
    fn angle(&self) -> i32 {
        self.angle
    }
}

pub trait Correct {
    fn correct<'a,'b>(x: &'a str, y: &'b str) -> Option<&'a str>;
}
impl<T: PartialEq> Correct for T{
    fn correct<'a,'b>(x: &'a str, y: &'b str) -> Option<&'a str>{
        if x == y {Some(x)}
        else {None}
    }
}