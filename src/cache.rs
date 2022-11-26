use std::collections::HashMap;
use std::fmt::Write;

pub struct Cache {
    set_num: i32,
    set_size: i32,
    line_size: i32,
    pub log_access: String,
    cache_blocks: Vec<Vec<i32>>,
    mem_reference: HashMap<String, i32>,
}

impl Cache{
    pub fn new( set_num: i32, set_size: i32, line_size: i32) -> Self {
        Self {
            set_num,
            set_size,
            line_size,
            log_access: init_log(set_num, set_size, line_size),
            cache_blocks: init_cache(set_num, set_size, line_size),
            mem_reference: HashMap::new(),
        }
    }

    pub fn access(&mut self, instruction_type: &String, size: &String, mem_address: &String){
        if check_request(size, hex_to_decimal(mem_address)) == false{
            return;
        }
        let cache_details_str = self.split_address(hex_to_binary(mem_address));
        let cache_details = binary_to_decimal(cache_details_str);
        if instruction_type.to_lowercase().eq("read"){
            for cache in self.cache_blocks.iter_mut(){
                //finds the correct index in the cache
                if cache[0] == cache_details[1]{
                    //checks if the index has been used already, if not it populates it
                    if cache[cache.len() - 1] == 0{
                        cache[1] = cache_details[0];
                        for offset in 2..cache.len(){
                            cache[offset] = 1;
                        }
                        if self.mem_reference.contains_key(mem_address){
                            *self.mem_reference.get_mut(mem_address).unwrap() += 1;
                        }else{
                            self.mem_reference.insert(mem_address.to_string(), 1);
                        }
                        write!(self.log_access, "read\t\t{}\t\t{}\t{}\t{}\tmiss\t\t{}\n\t",
                               mem_address, cache_details[0 as usize], cache_details[1 as usize],
                               cache_details[2 as usize], self.mem_reference.get(mem_address)
                                   .unwrap()).expect("Failure writing to string");
                    }else{
                        //check if address is already in cache
                        if cache[cache_details[2] as usize] == 1 && cache[1] == cache_details[0]{
                            *self.mem_reference.get_mut(mem_address).unwrap() += 1;
                            write!(self.log_access, "read\t\t{}\t\t{}\t{}\t{}\thit\t\t{}\n\t",
                                   mem_address, cache_details[0 as usize], cache_details[1 as usize],
                                   cache_details[2 as usize], self.mem_reference.get(mem_address)
                                       .unwrap()).expect("Failure writing to string");
                        }else{
                            cache[1] = cache_details[0];
                            for offset in 2..cache.len(){
                                cache[offset] = 1;
                            }
                            if self.mem_reference.contains_key(mem_address){
                                *self.mem_reference.get_mut(mem_address).unwrap() += 1;
                            }else{
                                self.mem_reference.insert(mem_address.to_string(), 1);
                            }
                            write!(self.log_access, "read\t\t{}\t\t{}\t{}\t{}\tmiss\t\t{}\n\t",
                                   mem_address, cache_details[0 as usize], cache_details[1 as usize],
                                   cache_details[2 as usize], self.mem_reference.get(mem_address)
                                       .unwrap()).expect("Failure writing to string");
                        }
                    }
                }
            }
        }else{
            //write code goes here.
        }
    }

    fn split_address(&self, address: String) -> Vec<String>{
        let address = address.chars().rev().collect::<String>();
        let mut tag: String = String::new();
        let mut index: String = String::new();
        let mut offset: String = String::new();
        let mut returnable: Vec<String> = vec![];
        let temp_line_size = fast_math::log2(self.line_size as f32);
        let temp_set_num = fast_math::log2(self.set_num as f32);
        let mut i = 0;
        while i < address.len(){
            if i < temp_line_size as usize{
                offset.push(address.chars().nth(i).unwrap());
            }else if i < (temp_line_size + temp_set_num) as usize{
                index.push(address.chars().nth(i).unwrap());
            }else{
                tag.push(address.chars().nth(i).unwrap());
            }
            i += 1;
        }
        returnable.push(tag.chars().rev().collect::<String>());
        returnable.push(index.chars().rev().collect::<String>());
        returnable.push(offset.chars().rev().collect::<String>());
        returnable
    }

    pub fn to_string(&self) -> String {
        self.log_access.clone()
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

fn hex_to_binary(word: &String) -> String{
    let returns = word.chars().map(to_binary).collect();
    fn to_binary(letter: char) -> &'static str{
        match letter.to_ascii_uppercase(){
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
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        }
    }
    returns
}

fn binary_to_decimal(cache_details: Vec<String>) -> Vec<i32> {
    let mut returns: Vec<i32> = vec![];
    for detail in cache_details{
        if detail.eq(""){
            returns.push(0);
        }else{
            returns.push(i32::from_str_radix(&detail, 2)
                .expect("Unable to convert from binary to decimal"))
        }
    }
    returns
}

fn hex_to_decimal(string: &String) -> i32 {
    let returns = i64::from_str_radix(string, 16).expect("Unable to convert from hex to decimal");
    returns as i32
}

fn init_cache(set_num: i32, set_size: i32, line_size: i32) -> Vec<Vec<i32>>{ //return type is broken.
    let mut returns = vec![];
    let sum_size = set_num * set_size;
    for index in 0..sum_size{
        let mut temp = vec![];
        for int in 0..line_size{
            if int == 0{
                temp.push(index);
            }else{
                temp.push(0);
            }
        }
        returns.push(temp);
    }
    returns
}

fn init_log(set_num: i32, set_size: i32, line_size: i32) -> String{
    let mut log_access = String::new();
    write!(log_access, "Cache Configuration\n\n").expect("Failure writing to string");
    write!(log_access, "{}", format_args!("\t{} {}-way set associative entries\n\tof line size {} \
        bytes\n\n\n", set_num , set_size.to_string(), line_size.to_string()))
        .expect("Failure writing to string");
    write!(log_access, "Results for Each Reference\n\n")
        .expect("Failure writing to string");
    write!(log_access, "Access Address\t Tag\tIndex Offset Result Memrefs\n\
        ------ -------- ------- ----- ------ ------ -------\n\t").expect("Failure writing to string");
    log_access
}