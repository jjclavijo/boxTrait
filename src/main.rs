mod boxtest;

use crate::boxtest::{Model,get_model,model_to_h};

fn main() {
    let models = vec!["white","powerlaw"];
    let ms: Vec<Box<dyn Model<f32>>> = 
        models.into_iter().map(|st| get_model(st)).collect();
    println!("{:?}", model_to_h(ms));
}
