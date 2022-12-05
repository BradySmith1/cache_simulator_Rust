use std::fmt::Write;
use std::process::exit;
use pad::{PadStr, Alignment};

pub struct Cache {
    set_num: i32,
    set_size: i32,
    line_size: i32,
    total_hits: f64,
    // field for total hits
    total_misses: f64,
    // field for total misses
    pub log_access: String,
    cache_blocks: Vec<Vec<i32>>,
}

impl Cache {
    pub fn new(set_num: i32, set_size: i32, line_size: i32, total_hits: f64, total_misses: f64)
               -> Self {
        Self {
            set_num,
            set_size,
            line_size,
            total_hits,
            total_misses,
            log_access: init_log(set_num, set_size, line_size),
            cache_blocks: init_cache(set_num, set_size, line_size),
        }
    }

    pub fn access(&mut self, instruction_type: &String, size: &String, mem_address: &String) {
        if check_request(size, hex_to_decimal(mem_address)) == false {
            return;
        }
        let cache_details_str = self.split_address(hex_to_binary(mem_address));
        let cache_details = binary_to_decimal(cache_details_str);
        if instruction_type.to_lowercase().eq("read") {
            //read code goes here.
            self.read_cache(&cache_details, mem_address);
        } else if instruction_type.to_lowercase().eq("write") {
            //write-back code goes here.
            self.write_cache(&cache_details, mem_address);
        } else {
            println!("Invalid instruction type {}", instruction_type);
            exit(1);
        }
    }

    fn read_cache(&mut self, cache_details: &Vec<i32>, mem_address: &String) -> i32 {
        let mut mem_reference: i32 = 0;
        let mut searches = 0;
        let mut cache_num = 0;
        while cache_num < self.cache_blocks.len() {
            //finds the correct index in the cache
            if self.cache_blocks[cache_num][0] == cache_details[1] {
                //checks if the index has been used already, if not it populates it
                if self.cache_blocks[cache_num][self.cache_blocks[cache_num].len() - 1] == 0 {
                    self.total_misses += 1.0;         // >>>>>>>> Updates total_misses for read
                    mem_reference += 1;
                    self.mem_to_cache(cache_num, cache_details, mem_address, mem_reference);
                    return mem_reference;
                } else {
                    //check if address is already in cache
                    if self.cache_blocks[cache_num][(cache_details[2] + 3) as usize] == 1 &&
                        self.cache_blocks[cache_num][1] == cache_details[0] {
                        self.total_hits += 1.0;       // >>>>>>>> Updates total_hits for read
                        write!(self.log_access, "{access}{}{}{}{}{miss_hit}{}\n",
                               mem_address.pad_to_width_with_alignment(9, Alignment::Right), cache_details[0 as usize].to_string().pad_to_width_with_alignment(8, Alignment::Right), cache_details[1 as usize].to_string().pad_to_width_with_alignment(6, Alignment::Right),
                               cache_details[2 as usize].to_string().pad_to_width_with_alignment(7, Alignment::Right), mem_reference.to_string().pad_to_width_with_alignment(8, Alignment::Right), access = "read".pad_to_width_with_alignment(6, Alignment::Right), miss_hit = "hit".pad_to_width_with_alignment(7, Alignment::Right))
                            .expect("Failure writing to string");
                        return mem_reference;
                    } else if self.set_size - 1 > searches {
                        //if the index is not in the cache, it checks the next index
                        searches += 1;
                        cache_num += 1;
                        continue;
                    } else {
                        self.total_misses += 1.0;    // >>>>>>> updates total misses for read
                        mem_reference += 1;
                        let mut beginning_cache = cache_num - (searches as usize);
                        for num in beginning_cache..cache_num {
                            if num + 1 <= cache_num {
                                self.cache_blocks[num + 1] = self.cache_blocks[num].clone();
                            }
                        }
                        self.mem_to_cache(beginning_cache, &cache_details, mem_address
                                          , mem_reference);
                        cache_num += 1;
                        return mem_reference;
                    }
                }
            } else {
                cache_num += 1;
            }
        }
        return mem_reference;
    }

    fn write_cache(&mut self, cache_details: &Vec<i32>, mem_address: &String) {
        let mut mem_reference: i32 = 0;
        let mut searches = 0;
        let mut cache_num = 0;
        while cache_num < self.cache_blocks.len() {
            //finds the correct index in the cache
            if self.cache_blocks[cache_num][0] == cache_details[1] {
                //write is a hit and rewrites the cache
                let hit = "hit".to_string();
                if self.cache_blocks[cache_num][1] == cache_details[0] &&
                    self.cache_blocks[cache_num][self.cache_blocks[cache_num].len() - 1] == 1 {
                    self.total_hits += 1.0;                 // >>>>>>>>> Update of total_hits for write
                    self.write_back_cache(cache_num, cache_details, mem_address, mem_reference, hit);
                    return;
                } else {
                    if self.set_size - 1 > searches {
                        //if the index is not in the cache, it checks the next index
                        searches += 1;
                        cache_num += 1;
                        continue;
                    } else {
                        //write miss
                        self.total_misses += 1.0;     // >>>>>>>> update of total_misses for write
                        let miss = "miss".to_string();
                        mem_reference = self.write_allocate_cache(cache_num, searches, cache_details);
                        self.write_back_cache(cache_num, &cache_details, mem_address, mem_reference, miss);
                        return;
                    }
                }
            } else {
                cache_num += 1;
            }
        }
    }

    fn write_allocate_cache(&mut self, cache_num: usize, searches: i32, cache_details: &Vec<i32>) -> i32 {
        let mut mem_reference = 1;
        //checks for dirty bit, if 1 then it has to write it back to memory before replacing
        if self.cache_blocks[cache_num][2] == 1 {
            mem_reference += 1;
        }
        let mut beginning_cache = cache_num - (searches as usize);
        for num in beginning_cache..cache_num {
            if num + 1 <= cache_num {
                self.cache_blocks[num + 1] = self.cache_blocks[num].clone();
            }
        }
        self.cache_blocks[beginning_cache][1] = cache_details[0];
        for offset in 3..self.cache_blocks[beginning_cache].len() {
            self.cache_blocks[beginning_cache][offset] = 1;
        }
        return mem_reference;
    }

    fn mem_to_cache(&mut self, cache_num: usize, cache_details: &Vec<i32>, mem_address: &String, mem_reference: i32) {
        self.cache_blocks[cache_num][1] = cache_details[0];
        for offset in 3..self.cache_blocks[cache_num].len() {
            self.cache_blocks[cache_num][offset] = 1;
        }
        write!(self.log_access, "{access}{}{}{}{}{miss_hit}{}\n",
               mem_address.pad_to_width_with_alignment(9, Alignment::Right), cache_details[0 as usize].to_string().pad_to_width_with_alignment(8, Alignment::Right), cache_details[1 as usize].to_string().pad_to_width_with_alignment(6, Alignment::Right),
               cache_details[2 as usize].to_string().pad_to_width_with_alignment(7, Alignment::Right), mem_reference.to_string().pad_to_width_with_alignment(8, Alignment::Right), access = "read".pad_to_width_with_alignment(6, Alignment::Right), miss_hit = "miss".pad_to_width_with_alignment(7, Alignment::Right))
            .expect("Failure writing to string");
        return;
    }

    fn write_back_cache(&mut self, cache_num: usize, cache_details: &Vec<i32>, mem_address: &String, mem_reference: i32, hit_miss: String) {
        for offset in 2..self.cache_blocks[cache_num].len() {
            self.cache_blocks[cache_num][offset] = 1;
        }
        write!(self.log_access, "{access}{}{}{}{}{}{}\n",
               mem_address.pad_to_width_with_alignment(9, Alignment::Right), cache_details[0 as usize].to_string().pad_to_width_with_alignment(8, Alignment::Right), cache_details[1 as usize].to_string().pad_to_width_with_alignment(6, Alignment::Right),
               cache_details[2 as usize].to_string().pad_to_width_with_alignment(7, Alignment::Right), hit_miss.pad_to_width_with_alignment(7, Alignment::Right), mem_reference.to_string().pad_to_width_with_alignment(8, Alignment::Right), access = "write".pad_to_width_with_alignment(6, Alignment::Right))
            .expect("Failure writing to string");
        return;
    }

    fn split_address(&self, address: String) -> Vec<String> {
        let address = address.chars().rev().collect::<String>();
        let mut tag: String = String::new();
        let mut index: String = String::new();
        let mut offset: String = String::new();
        let mut returnable: Vec<String> = vec![];
        let temp_line_size = fast_math::log2(self.line_size as f32);
        let temp_set_num = fast_math::log2(self.set_num as f32);
        let mut i = 0;
        while i < address.len() {
            if i < temp_line_size as usize {
                offset.push(address.chars().nth(i).unwrap());
            } else if i < (temp_line_size + temp_set_num) as usize {
                index.push(address.chars().nth(i).unwrap());
            } else {
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

    pub fn summary(&self) -> String {
        let mut summary: String = String::new();
        let mut total_accesses: f64 = self.total_hits + self.total_misses;
        write!(summary, "Simulation Summary Statistics\n").expect("Failure writing to string");
        write!(summary, "-----------------------------\n").expect("Failure writing to string");
        write!(summary, "{}", format_args!("Total hits       : {}\nTotal misses     : {}\nTotal accesses   : \
            {}\nHit ratio        : {}\nMiss ratio       : {}", self.total_hits, self.total_misses, total_accesses,
                                           self.total_hits / total_accesses, self.total_misses / total_accesses))
            .expect("Failure writing to string");
        summary
    }
}

fn check_request(size: &String, mem_add_num: i32) -> bool {
    let size_result = size.parse::<i32>().unwrap();
    if mem_add_num % size_result != 0 {
        println!("Memory Address is misaligned. Access will be ignored.");
        return false;
    }
    return true;
}

fn hex_to_binary(word: &String) -> String {
    let returns = word.chars().map(to_binary).collect();
    fn to_binary(letter: char) -> &'static str {
        match letter.to_ascii_uppercase() {
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
    for detail in cache_details {
        if detail.eq("") {
            returns.push(0);
        } else {
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

fn init_cache(set_num: i32, set_size: i32, line_size: i32) -> Vec<Vec<i32>> { //return type is broken.
    let mut returns = vec![];
    let mut index = 0;
    for _ in 0..set_num {
        for _ in 0..set_size {
            let mut temp = vec![];
            for int in 0..line_size + 4 {
                if int == 0 {
                    temp.push(index);
                } else {
                    temp.push(0);
                }
            }
            returns.push(temp);
        }
        index += 1;
    }
    returns
}

fn init_log(set_num: i32, set_size: i32, line_size: i32) -> String {
    let mut log_access = String::new();
    write!(log_access, "Cache Configuration\n\n").expect("Failure writing to string");
    write!(log_access, "{}", format_args!("\t{} {}-way set associative entries\n\tof line size {} \
        bytes\n\n\n", set_num, set_size.to_string(), line_size.to_string()))
        .expect("Failure writing to string");
    write!(log_access, "Results for Each Reference\n\n")
        .expect("Failure writing to string");
    write!(log_access, "Access Address\t Tag\tIndex Offset Result Memrefs\n\
        ------ -------- ------- ----- ------ ------ -------\n").expect("Failure writing to string");
    log_access
}