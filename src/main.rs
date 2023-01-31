fn main() {
    let weight_on_mars = 55.0;
    println!(
        "Weight on Mars: {}kg",
        calculate_weight_on_mars(weight_on_mars)
    );
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
