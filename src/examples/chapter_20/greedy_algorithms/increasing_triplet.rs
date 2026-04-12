use core::f64;
use std::ops::{Add, Sub, Mul, Div};

pub trait Numeric: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Copy + PartialOrd {}

impl Numeric for f32 {}
impl Numeric for f64 {}

impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}

pub fn increasing_triplet<T>(array: Vec<T>) -> bool
where 
    T: Numeric
{
    let mut lowest_price = array[0];
    let mut middle_price: Option<T> = None;

    for price in &array {
        if *price <= lowest_price {
            lowest_price = *price;
        } 
        
        /*
         * If the current price is higher than lowest price
         * but lower than middle price:
         */
        else if middle_price.is_none() || *price <= middle_price.unwrap() {
            middle_price = Some(*price);
        } 
        
        // * If the current price is higher than the middle price:
        else {
            return true;
        }
    }

    false
}