pub trait Pizza {
    fn description(&self) -> String;
    fn cost(&self) -> f32;
}

pub struct PlainPizza;

impl Pizza for PlainPizza {
    fn description(&self) -> String {
        "Thin dough".to_string()
    }

    fn cost(&self) -> f32 {
        4.0
    }
}

pub struct Mozzarella {
    pub pizza: Box<dyn Pizza>,
}

impl Pizza for Mozzarella {
    fn description(&self) -> String {
        format!("{}, Mozzarella", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + 1.0
    }
}

pub struct TomatoSauce {
    pub pizza: Box<dyn Pizza>,
}

impl Pizza for TomatoSauce {
    fn description(&self) -> String {
        format!("{}, Tomato Sauce", self.pizza.description())
    }

    fn cost(&self) -> f32 {
        self.pizza.cost() + 0.5
    }
}
