// simple inheritance of traits

trait User {
    fn name(&self) -> &str;
}
trait SendMsg: User {
    fn send(&self, msg: &str) {
        println!("Message from {}: {}", self.name(), msg)
    }
}
struct Admin {}

impl User for Admin {
    fn name(&self) -> &str {
        "admin Arsi"
    }
}

impl SendMsg for Admin {}
fn main() {
    Admin {}.send("Hello");
}
