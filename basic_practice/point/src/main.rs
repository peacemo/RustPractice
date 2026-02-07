struct Point<T> {
    x: T, 
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMix<T1, T2> {
    x: T1, 
    y: T2
}

impl<T1, T2> PointMix<T1, T2> {
    fn mix_up<T3, T4>(self, other: PointMix<T3, T4>) -> PointMix<T1, T4> {
        PointMix { x: self.x, y: other.y }
    }
}

fn main() {
    let p: Point<f32> = Point {x:1.1, y:2.2};

    println!("p.x: {}, p.y: {}", p.x(), p.y());

    println!("Distance: {}", p.distance_from_origin());

    let p_m_1 = PointMix {x: 1, y: 2.2};
    let p_m_2 = PointMix {x: 1, y: "r"};

    let p_m_3 = p_m_1.mix_up(p_m_2);

    println!("p_m_3: {}, {}", p_m_3.x, p_m_3.y)   
}
