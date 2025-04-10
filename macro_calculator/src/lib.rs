pub use json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;

    for food in foods {
        let kcal_str = &food.calories[1];
        let kcal: f64 = kcal_str
            .strip_suffix("kcal")
            .unwrap_or(kcal_str)
            .parse()
            .unwrap_or(0.0);

        total_calories += kcal * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
    }
    let round_value = |value: f64| -> f64 {
        let rounded = (value * 100.0).round() / 100.0;
        if (rounded * 10.0).fract() == 0.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    };

    json::object! {
        "cals": round_value(total_calories),
        "proteins": round_value(total_proteins),
        "fats": round_value(total_fats),
        "carbs": round_value(total_carbs),
    }
}
