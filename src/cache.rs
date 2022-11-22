use std::fmt::Write;
use crate::split_instruction;

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
            cache_blocks: init_cache(set_num, set_size, line_size),
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
        if check_request(size, hex_to_decimal(mem_address)) == false{
            return;
        }
        let cache_details_str = self.split_address(hex_to_binary(mem_address));
        let cache_details_int = binary_to_decimal(cache_details_str);
        if instruction_type.to_lowercase() == "read"{
            for cache in self.cache_blocks{
                0
            }
        }else{

        }
    }
    pub fn to_string(&self) -> String {
        let mut returns: String = String::new();
        returns
    }

    fn split_address(&self, address: String) -> Vec<String>{
        let mut tag: String = String::new();
        let mut index: String = String::new();
        let mut offset: String = String::new();
        let mut returnable: Vec<String> = vec![];
        let mut i = 0;
        while i < address.len(){
            let temp_line_size = fast_math::log2(self.line_size as f32);
            let temp_set_num = fast_math::log2(self.set_num as f32);
            if i < temp_line_size as usize{
                offset.push(address.chars().nth(i).unwrap());
            }else if i < (temp_line_size + temp_set_num) as usize{
                index.push(address.chars().nth(i).unwrap());
            }else{
                tag.push(address.chars().nth(i).unwrap());
            }
            i += 1;
        }
        returnable.push(tag);
        returnable.push(index);
        returnable.push(offset);
        returnable
    }
}

fn check_request(size: &String, mem_add_num: i32 ) -> bool{
    let size_result = size.parse::<i32>().unwrap();
    if mem_add_num % size_result != 0{
        println!("Memory Address is misaligned. Access will be ignored.");
        return false;
    }
    return true;
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

fn binary_to_decimal(cache_details: Vec<String>) -> Vec<i32> {
    let mut returns: Vec<i32> = vec![];
    for detail in cache_details{
        returns.push(i32::from_str_radix(&detail, 2).expect("Unable to convert from binary to decimal"))
    }
    returns
}

fn hex_to_decimal(string: &String) -> i32 {
    let mut returns: i32 = 0;
    for char in string.chars(){
        returns += match char{
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => 0,
        };
    }
    returns
}

fn init_cache(set_num: i32, set_size: i32, line_size: i32) -> vec![vec![<i32>]]{
    let mut returns = vec![];
    //have to initialize the cache.
}