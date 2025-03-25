/// # Chapter 4 - Enum & Patterns
///
/// Create a struct that holds info about a gun(gun type, recoil time, magazine size, extra) where `gun type` and
/// `extra` are enums and `extra` contains silencer, scope, extended mags and `None`.
/// Based on user input change the value of extra (may cause change in recoil time and magazine size).
fn main() {
    let mut pistol = Gun::new(GunType::Pistol, 0.5, 15);
    println!("Initial gun: {:?}", pistol);

    pistol.apply_extra(Extra::Silencer);
    println!("After silencer: {:?}", pistol);

    pistol.apply_extra(Extra::ExtendedMags);
    println!("After extended mags: {:?}", pistol);

    pistol.apply_extra(Extra::Scope);
    println!("After scope: {:?}", pistol);

    pistol.apply_extra(Extra::None);
    println!("After removing extra: {:?}", pistol);

    let mut rifle = Gun::new(GunType::Rifle, 0.3, 30);
    println!("Initial rifle: {:?}", rifle);

    rifle.apply_extra(Extra::ExtendedMags);
    println!("After extended mags: {:?}", rifle);

    rifle.apply_extra(Extra::None);
    println!("After removing extra: {:?}", rifle);

    let mut shotgun = Gun::new(GunType::Shotgun, 1.0, 8);
    println!("Initial shotgun: {:?}", shotgun);

    shotgun.apply_extra(Extra::Silencer);
    println!("After Silencer: {:?}", shotgun);

    shotgun.apply_extra(Extra::None);
    println!("After removing extra: {:?}", shotgun);

    let mut sniper = Gun::new(GunType::Sniper, 2.0, 5);
    println!("Initial sniper: {:?}", sniper);

    sniper.apply_extra(Extra::Scope);
    println!("After scope: {:?}", sniper);

    sniper.apply_extra(Extra::None);
    println!("After removing extra: {:?}", sniper);
}

#[derive(Debug, PartialEq)]
enum GunType {
    Pistol,
    Rifle,
    Shotgun,
    Sniper,
}

#[derive(Debug, PartialEq)]
enum Extra {
    Silencer,
    Scope,
    ExtendedMags,
    None,
}

#[derive(Debug)]
struct Gun {
    gun_type: GunType,
    recoil_time: f32,
    magazine_size: u32,
    extra: Extra,
}

impl Gun {
    fn new(gun_type: GunType, recoil_time: f32, magazine_size: u32) -> Gun {
        Gun {
            gun_type,
            recoil_time,
            magazine_size,
            extra: Extra::None,
        }
    }

    fn apply_extra(&mut self, extra: Extra) {
        self.extra = extra;
        match self.extra {
            Extra::Silencer => {
                // Silencer slightly increases recoil time
                self.recoil_time *= 1.1;
            }
            Extra::Scope => {
                // Scope slightly decreases recoil time
                self.recoil_time *= 0.9;
            }
            Extra::ExtendedMags => {
                // Extended mags increases magazine size
                self.magazine_size *= 2;
            }
            Extra::None => {
                // Return to original values.
                match self.gun_type {
                    GunType::Pistol => {
                        self.recoil_time = 0.5;
                        self.magazine_size = 15;
                    }
                    GunType::Rifle => {
                        self.recoil_time = 0.3;
                        self.magazine_size = 30;
                    }
                    GunType::Shotgun => {
                        self.recoil_time = 1.0;
                        self.magazine_size = 8;
                    }
                    GunType::Sniper => {
                        self.recoil_time = 2.0;
                        self.magazine_size = 5;
                    }
                }
            }
        }
    }
}
