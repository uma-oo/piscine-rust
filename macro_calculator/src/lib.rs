use json;
use json::object;


#[derive(Debug)]

pub struct Food {
    // expected public fields
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for i in 0..foods.len() {
        println!("{:?}", foods[i]);
        let real_carbs = foods[i].calories.1.find("kcal").unwrap();
        cals += &foods[i].calories.1[0..real_carbs].parse().unwrap()*foods[i].nbr_of_portions;
        carbs += foods[i].carbs * foods[i].nbr_of_portions;
        proteins += foods[i].proteins * foods[i].nbr_of_portions;
        fats += foods[i].fats * foods[i].nbr_of_portions;
    }

    object! {
        "cals": (cals*100.0).round()/100.0,
        "carbs":(carbs*100.0).round()/100.0, 
        "proteins":(proteins*100.0).round()/100.0, 
        "fats":(fats*100.0).round()/100.0,
    }
}
