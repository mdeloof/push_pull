use push_pull::*;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32
}

impl PullFrom<Vec2> for Vec3 {
    fn pull_from(&mut self, vec_2: &Vec2) {
        self.x = vec_2.x;
        self.y = vec_2.y
    }
}

fn main() {
    let vec_2 = Vec2 { x: -23, y: 48 };
    let mut vec_3 = Vec3 { x: 43, y: -64, z: 104};
    
    vec_3.pull_from(&vec_2);
    assert_eq!(vec_3, Vec3 { x: -23, y: 48, z: 104 });

    let vec_2 = Vec2 { x: 0, y: 0 };

    // PushInto has a blanket implementation for types that implement 
    // `PullFrom`
    vec_2.push_into(&mut vec_3);
    assert_eq!(vec_3, Vec3 { x: 0, y: 0, z: 104 });
}