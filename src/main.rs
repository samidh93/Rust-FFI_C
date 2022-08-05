// src/main.rs
/* Solution task Kopernikus
* author: Sami Dhiab
* date: 31.07.2022
*/

// module 
mod ffi {
    #[repr(C)]
    pub struct VehiclePose
    {
        pub x: f32,
        pub y: f32,
        pub yaw: f32, 
        pub id: u64
    }
// extern c functions
extern "C"
{
    pub static next_id: u64;
    fn create(x: f32, y: f32, yaw: f32) -> VehiclePose;
    fn translate(vehicles: &mut VehiclePose , n: u64, dx: f32, dy: f32);
    fn create_heap() -> Box<VehiclePose>;

}
    // safe public interface 
    pub fn create_vehicle(x: f32, y: f32, yaw: f32) -> VehiclePose
    {
        unsafe 
        {
            let vehicle = create(x, y, yaw);
            println!("vehicle: id:{} x: {}  y: {} yaw:{}", vehicle.id, vehicle.x, vehicle.y, vehicle.yaw);
            vehicle
        }
    }
    pub fn translate_vehicle(vehicles: &mut VehiclePose , n: u64, dx: f32, dy: f32)
    {
        unsafe 
        {
            translate(vehicles, n, dx, dy);
            println!("vehicle: id:{} x: {}  y: {} yaw:{}", (vehicles).id, (vehicles).x, (vehicles).y, (vehicles).yaw);

        }
    }
    pub fn create_vehicles_vector(n: usize) -> Vec<VehiclePose> 
    {
        let mut v : Vec<VehiclePose> = Vec::new(); 
        for p in 1..=n {
            v.push(create_vehicle(1.0, 2.0, 3.0));
        }
            v
    }

    // return an option
    pub fn create_heap_safe()-> Option<Box<VehiclePose>>
    {
        unsafe{
        let veh_heap = create_heap();
        Some(veh_heap)
        }  
    } 

    /* */
    pub fn print_veh_id(vehicles: Box<VehiclePose>)
    {
        unsafe{ println!("veh id {}",(*vehicles).id);}
       
    }
}
fn main() {
    //create 3 vehicles
    println!("creating 3 vehicles");
    let veh_1 = &mut ffi::create_vehicle(1.0, 2.0, 3.0);
    let veh_2 = &mut ffi::create_vehicle(1.5, 2.5, 3.5);
    let veh_3 = &mut ffi::create_vehicle(2.0, 3.0, 3.5);
    // translate them
    println!("translating the 3 vehicles");
    ffi::translate_vehicle(veh_1, 1, 1.0,2.0);
    ffi::translate_vehicle(veh_2, 1, 1.0,2.0);
    ffi::translate_vehicle(veh_3, 1, 1.0,2.0);
    // create vector with 10 same vehicle vector 
    println!("creating 10 same vehicles in a vector");
    let vehicles : Vec<ffi::VehiclePose>= ffi::create_vehicles_vector(10);
    // create veh on heap
    let veh = ffi::create_heap_safe().unwrap();
    // check if heap veh id updated
    println!("print id vehicle on heap");
    ffi::print_veh_id(veh);
    // create vector of custom vehicles
    println!("create random 10 vehicle in vector");
    let v = vec![ffi::create_vehicle(1.0, 2.0, 3.0),ffi::create_vehicle(2.0, 2.0, 3.0),ffi::create_vehicle(3.0, 2.0, 3.0),ffi::create_vehicle(1.0, 2.0, 3.0),ffi::create_vehicle(5.0, 2.0, 3.0),ffi::create_vehicle(6.0, 2.0, 3.0),ffi::create_vehicle(7.0, 2.0, 3.0),ffi::create_vehicle(8.0, 2.0, 3.0),ffi::create_vehicle(1.0, 2.0, 3.0),ffi::create_vehicle(9.0, 2.0, 3.0),ffi::create_vehicle(10.0, 2.0, 3.0)];

}

