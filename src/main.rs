
// today I think we're going to sort vectors
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html


fn sortint()
{
    // unsorted int vector
    let mut vec: Vec<i32> = vec![6, 3, 5, 8, 1, 4];

    println!("unsorted int vec: {:?}", vec);

    // sort int vector smallest to largest
    vec.sort();

    print!("sorted int vec: {:?} ", vec);
}

fn sortfloat()
{
    // unsorted float vector
    let mut vec: Vec<f64> = vec![1.1, 1.15, 5.5, 1.3, 4.3, 2.0];

    println!("unsorted float vec: {:?}", vec);

    // sort float vector smallest to largest
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("sorted float vec: {:?}", vec);
}

// create person struct
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person
{
    name: String,
    age: u32
}

// implement a new function for the Person struct
impl Person
{
    pub fn new(name: String, age: u32) -> Self 
    {
        Person 
        {
            name,
            age
        }
    }
}

fn sortstructs()
{
    // unsorted vector of people called couple
    let mut couple: Vec<Person> = vec![Person::new("Sam".to_string(), 18), Person::new("Melody".to_string(), 19)];

    println!("unsorted couple: {} (age {}) | {} (age {})", couple[0].name, couple[0].age, couple[1].name, couple[1].age);

    // sort by derived natural order (name and age)
    couple.sort();

    println!("derived sorted couple: {} (age {}) | {} (age {})", couple[0].name, couple[0].age, couple[1].name, couple[1].age);

    // sort by age
    couple.sort_by(|a, b| b.age.cmp(&a.age));

    println!("sorted by age couple: {} (age {}) | {} (age {})", couple[0].name, couple[0].age, couple[1].name, couple[1].age);

}

fn main()
{
    sortint();
    println!("\n");
    sortfloat();
    println!("\n");
    sortstructs();
}