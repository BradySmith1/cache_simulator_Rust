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

    pub fn access(&self, instruction_type: &String, size: &String, mem_address: &String){
        let mut mem_address = hex_to_binary(mem_address);
        //need to write a function to split up the mem address into tag, index, and offset based on the config
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
    fn hex_to_binary(string: &String) -> String {
        let mut returns: String = String::new();
        for char in string.chars(){
            let char= match char{
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'a' => "1010",
                'b' => "1011",
                'c' => "1100",
                'd' => "1101",
                'e' => "1110",
                'f' => "1111",
                _ => "0000",
            };
            write!(&mut returns, "{}", char.to_string()).expect("Failure writing to string");
        }
        returns
    }
}