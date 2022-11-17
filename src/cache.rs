use std::fmt::Write;

pub struct Cache {
    set_num: i32,
    set_size: i32,
    line_size: i32,
}

impl Cache{
    pub fn new(set_num: i32, set_size: i32, line_size: i32) -> Self{
        Self{
            set_num,
            set_size,
            line_size,
        }
    }

    pub fn to_string(&self) -> String {
        let mut returns: String = String::new();
        write!(&mut returns, "Cache Configuration\n\n").expect("Failure writing to string");
        write!(&mut returns, "{}", format_args!("\t{} {}-way set associative entries\n\tof line size {} \
        bytes\n\n\n", self.set_num , self.set_size.to_string(), self.line_size.to_string()))
            .expect("Failure writing to string");
        write!(&mut returns, "Results for Each Reference\n\n")
            .expect("Failure writing to string");
        returns
    }
}