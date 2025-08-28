
fn main () {

    enum Santim {
        Chichifo,
       Simuni,
       Amsa(String),
   }

   
   fn value_of_cent(cent:Santim) -> u8 {

    if let Santim::Simuni = cent {
        return 25
    } else {
        return 1
    }
}

}