//! LAST ACTUALITZATION: INCREASE THE SPEED
//! 
//! # Explaining
//! 
//! This is a method that allows calculate a prime number using a hash. 
//! The idea is really simple: we generate a number n passing the hash from 16-basis to a special basis, 
//! taken the value of tha char in 10-basis and multiply by 2^{i}, where i is the position of the char in the string. 
//! This number n, we calculate the rest module 31, obtaining an other number d. We find the numbers x=2^{d}-1 and y=2^{d+1}-1.
//! We call a function that returns the first prime nuber in [x,y]. This will be our choice. 
//!  
//! We apply module 31 for 2 reasons: 
//! 1. Every prime generate will be lower than 2^{32}.
//! 2. We can work with up 2^{512}. If every prime is lower than 2^{32}, we can use 16 or more hashes.
//! 
//! ```toml
//! 
//! [dependencies]
//! Gen_Prime = "1.1.5"
//! 
//! ```
//!  
//! # Examples
//! 
//! ------------------------------------------------------------------------------------------------------- 
//! EXAMPLE 1 
//! 
//! ```rust
//! extern crate primes;
//! 
//! 
//! # fn main() {
//!     let z:u128;
//!     println!("Introduce a hash");
//!     let mut a = String::new();
//!     std::io::stdin().read_line(&mut a).expect("Fail");
//!     let v : Vec<&str>=a.split("").collect();
//!     z=primes::hash_to_prime(v);
//!     println!("{}", z);
//! 
//! # } 
//! 
//! ``` 
//! Outputs:
//! -------------------------------------------------------------------------------------------------------
//! 
//! Introduce a hash
//! 
//! a123dfe4758bc27237a
//! 
//! 2147483647
//! 
//! -------------------------------------------------------------------------------------------------------
//! 
//! Introduce a hash
//! 
//! 074a4a47c445cf604b2ca687fed09fd8f3a78a16d20786b1a97aa6642fe0f87a8577a52cbace36bab6a6c40c3f1843be
//! 
//! 1073741827
//! 
//! -------------------------------------------------------------------------------------------------------
//! 
//! Introduce a hash
//! 
//! 00eccf559792ee42e3ea26e394e9ab52ce569e47504d44715102cc0c2ab4f542dc54c383e8e13d360ebc57c5ede5e64b
//! 
//! 16777259
//! 
//! -------------------------------------------------------------------------------------------------------
//! 
//! Introduce a hash
//! 
//! daa41c63b728ba70e6c377d7d17d9e53185fc720e7e6326626608eb1edc3735f67d963c303d92c9196583f3cd8739943
//!
//! 16553
//! 
//! -------------------------------------------------------------------------------------------------------
//! 
//! ## EXAMPLE 2
//! 
//! ```rust
//! use std::io;
//! use std::u128;
//! extern crate bigint;
//! extern crate primes;
//! extern crate rand;
//! use bigint::U512;
//! 
//! # fn main() { 
//!     let mut mult:bigint::U512;
//!     mult=U512::one();
//!     loop{
//!         println!("Introduce a hash");
//!         let mut a = String::new();
//!         std::io::stdin().read_line(&mut a).expect("Fail");
//!         let v : Vec<&str>=a.split("").collect();
//!         let n2=v.len() as u32;
//!         let res:u128;
//!         let mut v2=Vec::with_capacity((n2-3) as usize);
//!         for i in 0..n2-3 {
//!             i as usize;
//!             v2.push(v[(i+1) as usize]);
//!         } //this lines are to safe each char in one component of a vector, and to delete white spaces
//!         res=primes::hash_to_prime(v2); //Generating a prime
//!         mult=mult*(bigint::U512::from(res as usize)); //Product of the primes
//!         println!("Do you want to enter another hash?[Y/N]");
//!         let mut b = String::new();
//!         io::stdin().read_line(&mut b).expect("Fail");
//!         let b : char = b.trim().parse().expect("Please, input one choice");
//!         if b=='N' || b=='n'{
//!             break;
//!         }
//!     }
//!     println!("{}", mult);
//!  # }
//! 
//! ``` 
//! 
//! ## Output
//! 
//! Introduce a hash
//! 
//! daa41c63b728ba70e6c377d7d17d9e53185fc720e7e6326626608eb1edc3735f67d963c303d92c9196583f3cd8739943
//! 
//! 19
//! 
//! r:100
//! 
//! Do you want to enter another hash?[Y/N]
//! 
//! Y
//! 
//! Introduce a hash
//! 
//! 0907b5ff1b7d6f7b2639ca00565c8f42cbf7529b992f825b2676ec30c294b477b913476d6a508da9d195f1dacc893173
//! 
//! 23
//! 
//! r:39
//! 
//! Do you want to enter another hash?[Y/N]
//! 
//! Y
//! 
//! Introduce a hash
//! 
//! 4cb34fd780fbac405a433f428bd5672c0f7003cf1ae9d0675225c8d83783c0fc57e8b1f64a9bad799084f71381d1115e
//! 
//! 27
//! 
//! r:55
//! 
//! Do you want to enter another hash?[Y/N]
//! 
//! Y
//! 
//! Introduce a hash
//! 
//! fae89ae85b629f1d5263c7b5ad5035cc72c3cb02f95a9a242f5065f235611863f28fd4798818054e4cf69fd464e7f3b0
//! 
//! 8
//! 
//! r:49
//! 
//! Do you want to enter another hash?[Y/N]
//! 
//! Y
//! 
//! Introduce a hash
//! 
//! d0054fddab35b4aa7ab1356811f5c5ef7a1536ccca1ac76549b36030c875c4329d12f86b67591e3d9b3ca7f178861c94
//! 
//! 5
//! 
//! r:57
//! 
//! Do you want to enter another hash?[Y/N]
//! 
//! N
//! 
//! 110277345636720260972118701
//! 
//! where the number after each hash is the value mod 31 (d) and r is a rand number that calculates the rth prime in [x,y].
//! The last number is the product of all primes. Can you find one of the primes? ;)
//! 
    use std::u128;
    extern crate rand;
    use rand::Rng;

    pub fn hash_to_prime(v2: Vec<&str>) -> u128{
        let mut count:u128;
        let mut seed:u128;
        let mut res:u128;
        let mut n:u32;
        n=0;
        res=0;
        let n3=v2.len() as u32;
        for element in v2.clone(){
                
            count=hex(element);
            seed=count*u128::pow(2,(n3-1)-n);
            res=(res+seed)%31;
            n=n+1;
        }
        println!("{}", res);
        if res==0{
            res=31;    
        }
        let r = rand::thread_rng().gen_range(1, 101);
        let mut x=u128::pow(2,res as u32)-1;
        if x==0{
            x=1;
                
        }
        let y=u128::pow(2, (res+1) as u32)-1;
        let z:u128;
        z=private_find_prime(x, y, r);
        return z
            //println!("PRIME:{}", z);

    }
    fn hex (a: &str) -> u128 {
        let mut count:u128;
        count = 0;
        if a == "0"{
            count = 0;
        }    
        if a == "1"{
            count = 1;
        }    
        if a == "2"{
            count = 2;    
            }   

        if a == "3"{
            count = 3;
        }    
        if a == "4"{
            count = 4;
        }    
        if a == "5"{
            count = 5;
        }    
        if a == "6"{
            count = 6;
        }    
        if a == "7"{
            count = 7;
        }    
        if a == "8"{
            count = 8;
        }    
        if a == "9"{
            count = 9;
        }    
        if a == "a" || a=="A"{
            count = 10;
        }    
        if a == "b" || a=="B"{
            count = 11;
        }    
        if a == "c" || a=="C"{
            count = 12;
        }    
        if a == "d" || a=="D"{
            count = 13;
        }    
        if a == "e" || a=="E"{
            count = 14;
        }       
        if a == "f" || a=="F"{
            count = 15;
        }  
        if a == "\0"{
            count=0;
        } 
        count
    }
    fn private_find_prime (x: u128, y: u128, r: u32) -> u128{
        let mut n:u128;
        let mut i=0;
        let mut c;
        let mut d:u32;
        d=0;
        println!("r:{}", r);
        loop{
            if x>=8388607{ 
                n=x+i;
                c=its_prime(n); 
                if c==1{

                    break;
                }else{
                    i=i+1;
                }
            }else{
                n=x+i;
                c=its_prime(n);
                if c==1{
                    d=d+1;
                    i=i+1;
                    if d==r{
                        break;
                    }
                }else{
                    i=i+1;
                }
            }
        }
        if n==y{
            n=0;
        }
        if x==1{ 
            n=2;
        }
        n as u128
    }
    fn its_prime (x : u128) -> i32{
        let mut coun=0;
        let n;
        let mut i= 3;
        if x%2==0{
            n=0;
            return n;
        }
        while i < x{
        
            if x%i == 0{
                coun=coun+1;
            }
            i=i+2;
        }
        if coun == 0{
            n=1;
        }else{
            n=0;
        }
        n
    }
