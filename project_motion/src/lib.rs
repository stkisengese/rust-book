#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
    
    // Helper function to round to 1 decimal place
    fn round_to_1_decimal(value: f32) -> f32 {
        (value * 10.0).round() / 10.0
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment time by 1 second
        self.time += 1.0;
        
        // Calculate new position and velocity using physics equations
        // Position: p = p₀ + v₀·t + ½·a·t²
        // Velocity: v = v₀ + a·t
        // Note: gravity (9.8 m/s²) only affects the y-component
        
        // Update velocity (only y component changes due to gravity)
        self.actual_velocity.x = self.init_velocity.x;
        self.actual_velocity.y = ThrowObject::round_to_1_decimal(self.init_velocity.y - 9.8 * self.time);
        
        // Update position
        self.actual_position.x = self.init_position.x + self.init_velocity.x * self.time;
        self.actual_position.y = ThrowObject::round_to_1_decimal(
            self.init_position.y + self.init_velocity.y * self.time - 0.5 * 9.8 * self.time * self.time
        );
        
        // Check if the object has hit the ground (y <= 0)
        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}