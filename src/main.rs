// Define marker types
struct Set;
struct NotSet;

// Define the Builder struct with generics for all 4 properties
struct Builder<Name, Age, Height, Weight> {
    name: Option<String>,
    age: Option<u32>,
    height: Option<u32>,
    weight: Option<u32>,
    _name_marker: std::marker::PhantomData<Name>,
    _age_marker: std::marker::PhantomData<Age>,
    _height_marker: std::marker::PhantomData<Height>,
    _weight_marker: std::marker::PhantomData<Weight>,
}

// Define the final struct being built
struct Person {
    name: String,
    age: u32,
    height: u32,
    weight: u32,
}

// Initial state: none of the properties are set (NotSet for all)
impl Builder<NotSet, NotSet, NotSet, NotSet> {
    fn new() -> Self {
        Builder {
            name: None,
            age: None,
            height: None,
            weight: None,
            _name_marker: std::marker::PhantomData,
            _age_marker: std::marker::PhantomData,
            _height_marker: std::marker::PhantomData,
            _weight_marker: std::marker::PhantomData,
        }
    }
}

// Add method for setting `name`
impl<Age, Height, Weight> Builder<NotSet, Age, Height, Weight> {
    fn set_name(self, name: String) -> Builder<Set, Age, Height, Weight> {
        Builder {
            name: Some(name),
            age: self.age,
            height: self.height,
            weight: self.weight,
            _name_marker: std::marker::PhantomData,
            _age_marker: std::marker::PhantomData,
            _height_marker: std::marker::PhantomData,
            _weight_marker: std::marker::PhantomData,
        }
    }
}

// Add method for setting `age`
impl<Name, Height, Weight> Builder<Name, NotSet, Height, Weight> {
    fn set_age(self, age: u32) -> Builder<Name, Set, Height, Weight> {
        Builder {
            name: self.name,
            age: Some(age),
            height: self.height,
            weight: self.weight,
            _name_marker: std::marker::PhantomData,
            _age_marker: std::marker::PhantomData,
            _height_marker: std::marker::PhantomData,
            _weight_marker: std::marker::PhantomData,
        }
    }
}

// Add method for setting `height`
impl<Name, Age, Weight> Builder<Name, Age, NotSet, Weight> {
    fn set_height(self, height: u32) -> Builder<Name, Age, Set, Weight> {
        Builder {
            name: self.name,
            age: self.age,
            height: Some(height),
            weight: self.weight,
            _name_marker: std::marker::PhantomData,
            _age_marker: std::marker::PhantomData,
            _height_marker: std::marker::PhantomData,
            _weight_marker: std::marker::PhantomData,
        }
    }
}

// Add method for setting `weight`
impl<Name, Age, Height> Builder<Name, Age, Height, NotSet> {
    fn set_weight(self, weight: u32) -> Builder<Name, Age, Height, Set> {
        Builder {
            name: self.name,
            age: self.age,
            height: self.height,
            weight: Some(weight),
            _name_marker: std::marker::PhantomData,
            _age_marker: std::marker::PhantomData,
            _height_marker: std::marker::PhantomData,
            _weight_marker: std::marker::PhantomData,
        }
    }
}

// Final step: build method can only be called when all properties are set
impl Builder<Set, Set, Set, Set> {
    fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
            height: self.height.unwrap(),
            weight: self.weight.unwrap(),
        }
    }
}

fn main() {
    // Example 1: Set properties in one order
    let person1 = Builder::new()
        .set_name("Alice".to_string())
        .set_age(30)
        .set_height(160)
        .set_weight(55)
        .build();
    
    // Example 2: Set properties in a different order
    let person2 = Builder::new()
        .set_weight(75)
        .set_height(180)
        .set_name("Bob".to_string())
        .set_age(28)
        .build();
}
