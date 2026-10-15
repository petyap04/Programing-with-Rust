#[derive(Debug, Clone, Copy)]
enum VehicleKind {
    Car,
    Truck,
    Motorcycle,
    Bicycle,
}

#[derive(Debug)]
struct Vehicle {
    kind: VehicleKind,
    fuel: f64,     
    distance: f64,
}

#[derive(Debug)]
enum DriveError {
    NotEnoughFuel { needed: f64, have: f64 },
}

impl Vehicle {
    fn new(kind: VehicleKind, fuel: f64) -> Self {
        Vehicle { kind, fuel, distance: 0.0 }
    }

    fn fuel_consumption_per_km(&self) -> f64 {
        match self.kind {
            VehicleKind::Car => 0.07,
            VehicleKind::Truck => 0.15,
            VehicleKind::Motorcycle => 0.05,
            VehicleKind::Bicycle => 0.0,
        }
    }

    fn drive(&mut self, km: f64) -> Result<(), DriveError> {
        let per_km = self.fuel_consumption_per_km();
        let needed = per_km * km;
        if needed > self.fuel + 1e-12 {
            return Err(DriveError::NotEnoughFuel { needed, have: self.fuel });
        }
        self.fuel -= needed;
        self.distance += km;
        Ok(())
    }
}

fn main(){
    let mut car = Vehicle::new(VehicleKind::Car, 5.0); 
    match car.drive(50.0) {
        Ok(()) => println!("Car drove 50 km, rem fuel {:.3}, dist {:.3}", car.fuel, car.distance),
        Err(e) => println!("Drive failed: {:?}", e),
    }

    let mut bike = Vehicle::new(VehicleKind::Bicycle, 0.0);
    bike.drive(10.0).unwrap();
    println!("Bike drove 10 km, dist {:.3}", bike.distance);
}