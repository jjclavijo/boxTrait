
trait Model<T> {
    fn generate_h(&self) -> Vec<T>;
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Units {
    Mom,
    Msf,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Powerlaw {
    pub m: usize,
    pub sigma: f32,
    pub kappa: f32,
    pub dt: f32, 
    pub units: Units
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct WhiteNoise {
    m: usize,
    sigma: f32,
}

impl Model for White {

    fn generate_h(&self) -> Vec<f32> {
    // Create impulse function for White noise.
        let mut h: Vec<f32> = vec![0.0; self.m];
        h[0] = sigma
    }

}

impl Model for Powerlaw {
    fn generate_h(&self) -> Result<PowerLawReturn, ModelError> {
    let d = -self.kappa/2.0;
    // let gmsv = gauss_markov_scale_variance(sigma, d, units, dt).unwrap();
    let rpf = recursion_power_flicker_rw(self.m, d);
    //Ok(PowerLawReturn {gmsv, rpf})
    rpf
    }
}

// fn gauss_markov_scale_variance(
//     sigma: f32, 
//     spectral_density: f32, 
//     units: Units, 
//     dt: f32
//     ) -> Result<f32, ModelError> {
//     
//     let sigma2: f32 = match units {
//         Units::mom => (dt as f32/365.25).powf(0.5*spectral_density),
//         Units::msf => (dt as f32/3600.0).powf(0.5*spectral_density),
//     };
//     Ok(sigma*sigma2)
// }

fn recursion_power_flicker_rw(m: usize, d: f32) -> Vec<f32> {
    // Recursion to create impulse function for Powerlay, Flicker or RW noise
    // Flicker is Powerlaw with spectral density 0.5
    // RandomWalk is Powerlaw with spectral density 1.0
    let mut h: Vec<f32> = vec![0.0; m];
    
    h[0] = 1.0;
    let mut h0: f32 = 1.0;
    for i in 1..m {
        h[i] = (d+i as f32-1.0)/i as f32 * h0;
        h0 = (d+i as f32-1.0)/i as f32 * h0;
    }
    h
}

fn get_model(m: str) -> Box<dyn Model> {
    match m {
        "white" => White {m: 10, sigma: 1}
        "powerlaw" => Powerlaw {m: 10, sigma: 1, kappa: 0.5, dt: 1, units: Units::Mom }
    }
}
