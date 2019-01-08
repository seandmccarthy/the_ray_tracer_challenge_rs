mod rt;
use crate::rt::tuple::*;

fn main() {
    projectile_run();
}

struct World {
    gravity: Tuple,
    wind: Tuple
}

struct Projectile {
    position: Tuple,
    velocity: Tuple
}

fn tick(world: &World, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + world.gravity + world.wind;
    Projectile { position, velocity }
}

fn projectile_run() {
    let mut p = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalise(&vector(1.0, 1.0, 0.0))
    };
    let w = World { gravity: vector(0.0, -0.1, 0.0), wind: vector(-0.01, 0.0, 0.0) };

    loop {
        p = tick(&w, &p);
        if p.position.y <= 0.0 {
            break;
        }
        println!("{:?}", p.position);
    }
}
