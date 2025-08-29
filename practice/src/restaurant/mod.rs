pub mod front {

    pub struct User {
        name:String,
        age:i32
    }
    pub fn take_order () {
    }
    pub fn serve () {
        crate::restaurant::back::cook();
        

    }
} 
mod back {
    fn wash_dish () {
    }

    pub fn cook () {

        let user1 = super::front::User {
            name:String::from("value"),
        }

    }

    
}

