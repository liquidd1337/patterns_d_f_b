use patterns::decorator::{Pizza, PlainPizza, Mozzarella, TomatoSauce};


fn main() {
    let plain_pizza = Box::new(PlainPizza);
    let mozzarella_pizza = Box::new(Mozzarella { pizza: plain_pizza });
    let tomato_sauce_pizza = Box::new(TomatoSauce { pizza: mozzarella_pizza });

    println!("Description: {}", tomato_sauce_pizza.description());
    println!("Cost: ${:.2}", tomato_sauce_pizza.cost());
}
