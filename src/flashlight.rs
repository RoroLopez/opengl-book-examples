/// Structure for flashlight interactions
pub mod flashlight {
    pub struct FlashLight {
        is_on: bool
    }
    
    impl FlashLight {
        pub fn new(is_on: bool) -> Self {
            FlashLight {
                is_on
            }
        }
        
        pub fn toggle(&mut self) {
            self.is_on = !self.is_on;
        }
        
        pub fn get_light(&self) -> Vec<f32> {
            if self.is_on {
                return vec![1.0, 1.0, 1.0].to_owned()
            }
            vec![0.0, 0.0, 0.0].to_owned()
        }
    }
}