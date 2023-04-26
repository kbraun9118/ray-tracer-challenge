use crate::{color::{Color, Colors}, point_light::PointLight, tuple::Tuple, util::eq_f64};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    /**
       Combine the surface color with the light's color / intensity.

       Find the direction to the light source.

       Compute the ambient contribution.

       light_dot_normal represents the cosine of the angle between the
       light vector and the normal vector. A negative number means the
       light is on the same side of the surface.

       Compute the diffuse contribution.

       reflect_dot_eye represents the cosine of the angle between the
       reflection vector and the eye vector. A negative number means the
       light reflects away from the eye.

       Compute the specular contribution.

       Add the three contributions together to get the final shading.
    */
    pub fn lighting(
        &self,
        light: PointLight,
        point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
    ) -> Color {
        let effective_color = self.color() * light.intensity();

        let light_v = (light.position() - point).normalize();

        let ambient = effective_color * self.ambient();

        let light_dot_normal = light_v * normal_v;

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Colors::Black.into(), Colors::Black.into())
        } else {
            let diffuse = effective_color * self.diffuse() * light_dot_normal;

            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v * eye_v;

            if eq_f64(0.0, reflect_dot_eye) || reflect_dot_eye < 0.0 {
                (diffuse, Colors::Black.into())
            } else {
                let factor = reflect_dot_eye.powf(self.shininess());
                (diffuse, light.intensity() * self.specular() * factor)
            }
        };

        return ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Colors::White.into(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && eq_f64(self.ambient, other.ambient)
            && eq_f64(self.diffuse, other.diffuse)
            && eq_f64(self.specular, other.specular)
            && eq_f64(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(Color::from(Colors::White), m.color());
        assert_eq!(0.1, m.ambient());
        assert_eq!(0.9, m.diffuse());
        assert_eq!(0.9, m.specular());
        assert_eq!(200.0, m.shininess());
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::origin();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Colors::White.into());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Tuple::origin();

        let eye_v = Tuple::vector(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Colors::White.into());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::origin();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Colors::White.into());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::origin();

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Colors::White.into());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}