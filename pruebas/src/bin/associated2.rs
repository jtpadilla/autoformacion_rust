struct Container(i32, i32);

// El trail verifica si los 2 items estan almacenados dentro del contenedor.
// Tambien recupera y primer y el ultimo valor.
trait Contains {
    // Definir aqui los tipos genericos para que los puedan utilizar los metodos.
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Especifica que tipos son 'A' y 'B'.
    // Si el tip ode entrada es `Container(i32, i32)`, los
    // tipos 'output' seran determinados como 'i32' y 'i32'.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    /*
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    */

    // Toma el primer numero.
    fn first(&self) -> i32 { self.0 }

    // Toma el ultimo numero.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}

