// bringing the module auxiliary_building into scope
mod auxiliary_building;
mod tower_building;
mod utils;

// instead of writing full paths, we use the name "aux"
// henceforth in the file
use auxiliary_building::auxiliary as aux;
use tower_building::tower;

fn main() {
    println!("Startup of aux and tower");

    // a function within the auxiliary_building module
    aux::generate_energy();
    tower::start_consumption();

    auxiliary_building::plug::device::do_it();
    auxiliary_building::plug::other_device::do_it_other();
}
