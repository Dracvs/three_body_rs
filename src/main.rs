#[derive(Debug, Clone, Copy)]
struct Body {
    mass: f64,
    position: (f64, f64),
    velocity: (f64, f64),
}

impl Body {
    fn new(position: (f64, f64)) -> Self {
        Body {
            mass: 1.0,
            velocity: (0.0, 0.0),
            position,
        }
    }
}

struct Step {
    time: f64,
    step_id: u32,
    bodies: [Body; 3],
}

struct Simulation {
    bodies: [Body; 3],
}

const TIME_STEP: f64 = 0.5;
const STEPS: usize = 100_000;
const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

fn main() {
    let mut first = Body::new((0.3089693008, 0.4236727692));
    let mut second = Body::new((-0.5, 0.0));
    let mut third = Body::new((0.5, 0.0));

    // let mut steps = Vec::<Step>::with_capacity(STEPS);

    for n in 0..STEPS {
        let mut new_step = Step {
            step_id: n as u32,
            time: (n as f64) * TIME_STEP,
            bodies: [first, second, third],
        };

        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    let a = &new_step.bodies[j];
                    let mut b = new_step.bodies[i];

                    let dx = a.position.0 - b.position.0;
                    let dy = a.position.1 - b.position.1;
                    let r = (dx * dx + dy * dy).sqrt();
                    let force = GRAVITATIONAL_CONSTANT * b.mass * a.mass / r / r;
                    let angle = dy.atan2(dx);
                    let fx = force * angle.cos();
                    let fy = force * angle.sin();
                    b.velocity.0 += fx / b.mass * TIME_STEP;
                    b.velocity.1 += fy / b.mass * TIME_STEP;

                    new_step.bodies[i] = b;
                }
            }
        }

        for body in new_step.bodies.iter_mut() {
            body.position.0 += body.velocity.0 * TIME_STEP;
            body.position.1 += body.velocity.1 * TIME_STEP;
        }

        // report current state
        if n % 1_000 == 0 {
            println!("{:.06}, {:.06}", first.position.0, first.position.1);
        }

        first = new_step.bodies[0];
        second = new_step.bodies[1];
        third = new_step.bodies[2];
    }
}
