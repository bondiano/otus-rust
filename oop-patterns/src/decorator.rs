pub trait Coffee {
    fn cost(&self) -> f32;
    fn description(&self) -> &str;
}

pub struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f32 {
        1.0
    }

    fn description(&self) -> &str {
        "Simple coffee"
    }
}

pub struct Milk<'a> {
    coffee: &'a dyn Coffee,
}

impl Milk<'_> {
    pub fn new(coffee: &dyn Coffee) -> Milk {
        Milk { coffee }
    }
}

impl Coffee for Milk<'_> {
    fn cost(&self) -> f32 {
        self.coffee.cost() + 0.5
    }

    fn description(&self) -> &str {
        "Coffee with milk"
    }
}

pub struct Sugar<'a> {
    coffee: &'a dyn Coffee,
}

impl Sugar<'_> {
    pub fn new(coffee: &dyn Coffee) -> Sugar {
        Sugar { coffee }
    }
}

impl Coffee for Sugar<'_> {
    fn cost(&self) -> f32 {
        self.coffee.cost() + 0.2
    }

    fn description(&self) -> &str {
        "Coffee with sugar"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_coffee() {
        let coffee = SimpleCoffee;
        assert_eq!(coffee.cost(), 1.0);
        assert_eq!(coffee.description(), "Simple coffee");
    }

    #[test]
    fn test_coffee_with_milk() {
        let coffee = SimpleCoffee;
        let coffee_with_milk = Milk::new(&coffee);
        assert_eq!(coffee_with_milk.cost(), 1.5);
        assert_eq!(coffee_with_milk.description(), "Coffee with milk");
    }

    #[test]
    fn test_coffee_with_sugar() {
        let coffee = SimpleCoffee;
        let coffee_with_sugar = Sugar::new(&coffee);
        assert_eq!(coffee_with_sugar.cost(), 1.2);
        assert_eq!(coffee_with_sugar.description(), "Coffee with sugar");
    }

    #[test]
    fn test_coffee_with_milk_and_sugar() {
        let coffee = SimpleCoffee;
        let coffee_with_milk = Milk::new(&coffee);
        let coffee_with_milk_and_sugar = Sugar::new(&coffee_with_milk);
        assert_eq!(coffee_with_milk_and_sugar.cost(), 1.7);
        assert_eq!(
            coffee_with_milk_and_sugar.description(),
            "Coffee with sugar"
        );
    }
}
