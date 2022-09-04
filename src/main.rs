fn main() {
    let un :i32 = 0;
    let deux :i32 = 1;
    let nombre :i32 = 12;
    while nombre != 0 {
        let un = deux;
        let deux = deux + un;
        let nombre = nombre - 1;
        println!("rang {} valeur {}", nombre, un);
    }
}
