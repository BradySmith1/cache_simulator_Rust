use std::fmt::Write;

pub struct Cache {
    set_num: i32,
    set_size: i32,
    line_size: i32,
    pub log_access: String,
    cache_blocks: Vec<Vec<i32>>,
}

impl Cache{
    pub fn new( set_num: i32, set_size: i32, line_size: i32) -> Self {
        Self {
            set_num,
            set_size,
            line_size,
            log_access: String::new(),
            cache_blocks: vec![vec![]],
        }
    }

    pub fn init_log(&mut self){
        write!(&mut self.log_access, "Cache Configuration\n\n").expect("Failure writing to string");
        write!(&mut self.log_access, "{}", format_args!("\t{} {}-way set associative entries\n\tof line size {} \
        bytes\n\n\n", self.set_num , self.set_size.to_string(), self.line_size.to_string()))
            .expect("Failure writing to string");
        write!(&mut self.log_access, "Results for Each Reference\n\n")
            .expect("Failure writing to string");
        write!(&mut self.log_access, "Access Address\t Tag\tIndex Offset Result Memrefs\n\
        ------ -------- ------- ----- ------ ------ -------").expect("Failure writing to string");
    }

    pub fn access(&self, instruction_type: &String, size: i32, mem_address: i32){
        if instruction_type.to_lowercase() == "read"{
            read_cache();
        }else{
            write_cache();
        }
        fn read_cache(){

        }
        fn write_cache(){
            println!("write");
        }
    }
    pub fn to_string(&self) -> String {
        let mut returns: String = String::new();
        returns
    }
}