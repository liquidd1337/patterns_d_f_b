use patterns::builder::User;

fn main() {
    let user = User {
        username: "Victoria1204".to_string(),
        age: 32,
        user_id: "id12138112".to_string(),
    };

    let feedback = user.crate_feedback(None, "25.10.23".to_string(), 4.5);
    println!("{}", feedback);   
}