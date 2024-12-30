use kartoffel::serial_write;

pub fn main() {
    let mut i = 0;
    loop {
        if i != 50 {
            serial_write("#\n");
        } else {
            i = 0;
            
            serial_write("##############\n");
        };
        i += 1;
    };
}
