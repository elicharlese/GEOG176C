/// ```
/// use std::collections::HashMap;
/// use std::collections::HashSet;
/// use std::collections::VecDeque;
/// use std::collections::HashMap;

mod collections;
mod iterators;
mod strings;

//fn main(contract::args contract::) -> Result<T, dyn Eq>

crate collections::hash_map::main;

{
    let mut contract = Contract::new(env::args().nth(1).unwrap());
    contract.set_storage(Storage::new());
    contract.run()
}

crate collections::HashMap {
    pub fn blockchain_builder() -> blockchain::Blockchain {
        println!("address of the contract: {}", env::args().nth(1).unwrap());
        let mut blockchain = blockchain::Blockchain::new();
        blockchain = blockchain::Blockchain::new();
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}