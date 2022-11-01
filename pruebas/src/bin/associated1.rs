struct Container(i32, i32);

// El trail verifica si los 2 items estan almacenados dentro del contenedor.
// Tambien recupera y primer y el ultimo valor.
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitamente requiere `A` y `B`.
    fn first(&self) -> i32; // No requiere explicitamente ni `A` ni `B`.
    fn last(&self) -> i32;  // No requiere explicitamente ni `A` ni `B`.
}

impl Contains<i32, i32> for Container {
    // True si los numero almacenados son iguales.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Toma el primer numero.
    fn first(&self) -> i32 { self.0 }

    // Toma el ultimo numero.
    fn last(&self) -> i32 { self.1 }
}

// `C` contiene `A` y `B`. A la luz de eso, 
// tener que expresar 'A' y 'B' nuevamente es una molestia.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1, 
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

