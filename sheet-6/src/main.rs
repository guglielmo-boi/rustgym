use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;

/*
    2. Create a struct Car with the following fields:
    model: String, year: u32, price: u32, rent: bool.
    Create a struct CarDealer with a field that is a vector of Car .
    Create a struct User with a field that is an Option of Car .
    Implement the following methods for CarDealer :
    new that takes a vector of Car and returns a CarDealer
    add_car that takes a Car and adds it to the vector of Car
    print_cars that prints all the cars
    rent_user that takes a mutable reference to a User and a model: String, that
    identify the car, and assigns the car to the user and set the rent field to true. If the
    car is not found, print "Car not found".
    The car must be the same present in the vector of CarDealer and into the car field
    of the User.
    end_rental that takes a mutable reference to a User and set the rent field to false.
    If the user has no car, print "User has no car".
    Implement the new and default method for Car
    Implement the print_car method for User that prints the car if it is present, otherwise
    print "User has no car"
*/
#[derive(Debug)]
struct Car
{
    model: String,
    year: u32,
    price: u32,
    rent: bool
}

impl Car
{
    fn new(model: String, year: u32, price: u32, rent: bool) -> Car {
        Car {
            model: model,
            year: year,
            price: price,
            rent: rent
        }
    }

    fn default() -> Car {
        Car {
            model: String::new(),
            year: 0,
            price: 0,
            rent: false
        }
    }
}

struct CarDealer
{
    cars: Vec<Rc<RefCell<Car>>>
}

impl CarDealer
{
    fn new(cars: Vec<Rc<RefCell<Car>>>) -> CarDealer {
        CarDealer {
            cars: cars
        }
    }

    fn add_car(&mut self, car: Car) {
        self.cars.push(Rc::new(RefCell::new(car)))
    }

    fn print_cars(&self) {
        println!("{:?}", self.cars);
    }

    fn rent_user(&mut self, user: &mut User, model: &String) {
        let car = self.cars.iter_mut().find(|c| c.borrow().model.eq(model));

        match car {
            Some(c) => {
                if c.borrow().rent {
                    println!("Car already rent!");
                } else {
                    c.as_ref().borrow_mut().rent = true;
                    user.car = Some(c.clone())
                }
            },
            None => {
                println!("Car not found!");
            }
        }
    }

    fn end_rental(&mut self, user: &mut User) {
        match user.car.as_ref() {
            Some(c) => {
                c.as_ref().borrow_mut().rent = false;
                *user.car.borrow_mut() = None;
            },
            None => {
                println!("User has no car!");
            }
        }
    }
}

struct User
{
    car: Option<Rc<RefCell<Car>>>
}

impl User
{
    fn print_car(&self) {
        match self.car.as_ref() {
            Some(c) => {
                println!("{:?}", c);
            },
            None => {
                println!("User has no car!");
            }
        }
    }
}


/*
    3. Write the trait Sound that defines a method  make_sound  that returns a  String .
    Create some structs that implement the  Sound  trait (animals).
    Create a list of trait objects that implement the  Sound  trait via the struct
    FarmCell.
    The struct  FarmCell  should have a field  element  containing the trait object and a
    field  next  that holds an optional pointer to another  FarmCell .
    Implement the methods:
    new  for the struct  FarmCell  that takes a trait object and returns a new
    FarmCell.
    insert  for the struct  FarmCell  that takes a trait object and inserts it into the list.
    Implement the trait  Sound  for the struct  FarmCell  that returns the concatenation of
    the  make_sound  methods of all the elements in the list.
*/
trait Sound
{
    fn make_sound(&self) -> String;
}

struct Duck;

impl Sound for Duck
{
    fn make_sound(&self) -> String {
        String::from("Quack!")
    }
}

struct Dog;

impl Sound for Dog
{
    fn make_sound(&self) -> String {
        String::from("Bau!")
    }
}

struct Cat;

impl Sound for Cat 
{
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

struct FarmCell
{
    next: Option<Box<FarmCell>>,
    element: Box<dyn Sound>
}

impl FarmCell
{
    fn new(element: Box<dyn Sound>) -> FarmCell {
        FarmCell {
            next: None,
            element: element
        }
    }

    fn insert(&mut self, next: Box<dyn Sound>) {
        self.next = Some(Box::new(FarmCell::new(next)))
    }
}

impl Sound for FarmCell
{
    fn make_sound(&self) -> String {
        match &self.next {
            Some(fc) => {
                self.element.make_sound() + " " + &fc.make_sound()
            }, 
            None => self.element.make_sound()
        }
    }
}


/*
    4. Create the struct  PublicStreetlight  with the fields  id,  on  and  burn_out : it represent a
    public light, with its id, if it is on or off and if it is burned out or not.
    Create the struct  PublicIllumination  with the field  lights  that is a vector of
    PublicStreetlight .
    Implement the methods new and default for  PublicStreetlight  and
    PublicIllumination . Then implement the Iterator trait for  PublicIllumination  that
    returns the lights in order of permit the public operators to change them. The
    iterator must remove the burned out lights from the vector.
*/
struct PublicStreetlight
{
    id: u32,
    on: bool,
    burn_out: bool
}

impl PublicStreetlight
{
    fn new(id: u32, on: bool, burn_out: bool) -> PublicStreetlight {
        PublicStreetlight {
            id: id,
            on: on,
            burn_out: burn_out
        }
    }

    fn default() -> PublicStreetlight {
        PublicStreetlight {
            id: 0,
            on: false,
            burn_out: false
        }
    }
}

struct PublicIllumination
{
    lights: Vec<PublicStreetlight>
}

impl Iterator for PublicIllumination
{
    type Item = PublicStreetlight;

    fn next(&mut self) -> Option<Self::Item> {
        self.lights.iter_mut().position(|t| !t.burn_out).map(|i| self.lights.remove(i))
    }
}


/*
    6. Create a struct named  EntangledBit .
    Wen two bits  b1  and  b2 are entangled with each-other they are connected,
    meanings that they will always have the same value.
    A bit can be entangled with any number of other bits (including 0)
    implement the following functionalities:
    implement the Default trait for  EntangledBit that return a bit set to 0, entangled
    with 0 other bits.
    implement the methods set (set the bit to 1), reset (set the bit to 0) and get
    (return true or false) to manipulate a bit.
    implement a method  entangle_with(&self, other: &mut Self)  that entangle
    other  to  self .
    if  other  is entangled with other bits it gets "un-entangled".
    other 's value gets overwritten by the value of  self.
*/
struct EntangledBit
{
    bit: Rc<RefCell<bool>>
}

impl EntangledBit
{
    fn set(&mut self) {
        *self.bit.as_ref().borrow_mut() = true
    }

    fn reset(&mut self) {
        *self.bit.as_ref().borrow_mut() = false
    }

    fn get(&self) -> bool {
        *self.bit.as_ref().borrow()
    }

    fn entangle_with(&self, other: &mut EntangledBit) {
        other.bit = self.bit.clone()
    }
}

impl Default for EntangledBit
{
    fn default() -> EntangledBit {
        EntangledBit { 
            bit: Rc::new(RefCell::new(false))
        }  
    }
}


fn main()
{
    let mut dealer = CarDealer::new(vec![]);
    dealer.add_car(Car { model: String::from("Skoda Fabia"), year: 2023, price: 19000, rent: false });
    dealer.add_car(Car { model: String::from("Skoda Octavia"), year: 2023, price: 32000, rent: false });
    let mut user = User { car: None };
    dealer.rent_user(&mut user, &String::from("Skoda Fabia"));
    user.print_car();
    dealer.end_rental(&mut user);
    user.print_car();

    let mut cell_0 = FarmCell::new(Box::new(Duck{}));
    cell_0.insert(Box::new(Dog{}));
    println!("{}", cell_0.make_sound());

    let mut bit_0 = EntangledBit::default();
    let mut bit_1 = EntangledBit::default();
    bit_0.entangle_with(&mut bit_1);
    bit_0.set();
    println!("{}", bit_1.get());
}