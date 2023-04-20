use ray_tracer_challenge::{
    canvas::Canvas,
    error::RayTraceResult,
    intersection::{
        ray::Ray,
        shape::{Shape, Sphere},
    },
    transformation::Transformation,
    tuple::Tuple, color::Color,
};

fn main() -> RayTraceResult<()> {
    let mut c = Canvas::new(400, 400);
    let mut sphere = Sphere::new();
    sphere.with_transformation(
        Transformation::identity()
            .scale(50.0, 50.0, 50.0)
            .translation(200.0, 200.0, -300.0),
    );

    for y in 0..400 {
        for x in 0..400 {
            let r = Ray::try_new(
                Tuple::point(200.0, 200.0, -500.0),
                Tuple::vector(-200.0 + x as f64, -200.0 + y as f64, 500.0),
            )?;

            c[(x,y)] = if sphere.intersects(r).len() > 0 {
                Color::red()
            } else {
                Color::black()
            };
        }
    }

    c.save("spehere_shadow")?;

    Ok(())
}