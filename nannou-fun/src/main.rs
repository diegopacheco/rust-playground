use nannou::prelude::*;

struct Model {
    particles: Vec<Particle>,
    time: f32,
}

struct Particle {
    position: Vec2,
    color: Hsla,
    size: f32,
    angle: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 800)
        .title("Nannou Particle Visualization")
        .view(view)
        .build()
        .unwrap();

    let mut particles = Vec::new();
    for i in 0..200 {
        let angle = (i as f32 / 200.0) * TAU * 3.0;
        let radius = 50.0 + (i as f32 * 1.5);
        particles.push(Particle {
            position: vec2(angle.cos() * radius, angle.sin() * radius),
            color: hsla(i as f32 / 200.0, 0.8, 0.6, 0.8),
            size: random_range(3.0, 12.0),
            angle,
        });
    }

    Model { particles, time: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time += 0.02;

    for (i, particle) in model.particles.iter_mut().enumerate() {
        let base_angle = (i as f32 / 200.0) * TAU * 3.0;
        let wave = (model.time + i as f32 * 0.1).sin() * 30.0;
        let radius = 80.0 + (i as f32 * 1.2) + wave;
        
        let spiral_speed = 0.5;
        particle.angle = base_angle + model.time * spiral_speed;
        
        particle.position.x = particle.angle.cos() * radius;
        particle.position.y = particle.angle.sin() * radius;
        
        let hue = ((i as f32 / 200.0) + model.time * 0.1) % 1.0;
        particle.color = hsla(hue, 0.9, 0.55, 0.85);
        
        particle.size = 4.0 + ((model.time * 2.0 + i as f32 * 0.05).sin() * 4.0).abs();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.rect()
        .w_h(1200.0, 800.0)
        .color(srgba(0.02, 0.02, 0.05, 0.15));

    let center_pulse = (model.time * 1.5).sin() * 0.3 + 0.7;
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(60.0 * center_pulse)
        .color(hsla(model.time * 0.1 % 1.0, 0.9, 0.5, 0.6));

    for ring in 0..5 {
        let ring_radius = 100.0 + ring as f32 * 80.0;
        let ring_wave = (model.time + ring as f32 * 0.5).sin() * 10.0;
        draw.ellipse()
            .x_y(0.0, 0.0)
            .radius(ring_radius + ring_wave)
            .no_fill()
            .stroke_weight(2.0)
            .stroke(hsla((ring as f32 * 0.15 + model.time * 0.05) % 1.0, 0.7, 0.5, 0.3));
    }

    for particle in &model.particles {
        draw.ellipse()
            .xy(particle.position)
            .radius(particle.size)
            .color(particle.color);
        
        let glow_color = hsla(
            particle.color.hue.to_positive_degrees() / 360.0,
            0.9,
            0.6,
            0.15,
        );
        draw.ellipse()
            .xy(particle.position)
            .radius(particle.size * 2.5)
            .color(glow_color);
    }

    for i in 0..12 {
        let angle = (i as f32 / 12.0) * TAU + model.time * 0.3;
        let length = 350.0 + (model.time * 2.0 + i as f32).sin() * 50.0;
        let end = vec2(angle.cos() * length, angle.sin() * length);
        draw.line()
            .start(vec2(0.0, 0.0))
            .end(end)
            .weight(1.5)
            .color(hsla((i as f32 / 12.0 + model.time * 0.1) % 1.0, 0.6, 0.4, 0.25));
    }

    draw.to_frame(app, &frame).unwrap();
}
