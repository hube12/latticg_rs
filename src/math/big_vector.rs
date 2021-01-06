use crate::math::big_fraction::BigFraction;
use num_bigint::BigInt;
use num_traits::Zero;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BigVector {
    numbers: Vec<BigFraction>,
    dimension: usize,
    start_pos: usize,
    step: usize,
}

impl BigVector {
    pub fn new(number: &[BigFraction]) -> BigVector {
        BigVector {
            numbers: number.to_vec(),
            dimension: number.len(),
            start_pos: 0,
            step: 1,
        }
    }

    pub fn new_int(number: &[BigInt]) -> BigVector {
        BigVector {
            numbers: number
                .to_vec()
                .iter()
                .map(|x| BigFraction::new_integer(x.clone()).expect("Can not fail"))
                .collect(),
            dimension: number.len(),
            start_pos: 0,
            step: 1,
        }
    }

    pub fn get(&self, index: usize) -> Result<BigFraction, String> {
        if index > self.dimension {
            return Err(format!("Index {}, dimension {}", index, self.dimension));
        }
        let res: Option<&BigFraction> = self.numbers.get((self.step * index + self.start_pos) as usize);
        if res.is_none() {
            return Err(format!("ERROR NOT POSSIBLE: Index {}, dimension {}", index, self.dimension));
        }
        return Ok(res.unwrap().clone());
    }

    pub fn set(&mut self, index: usize, value: BigFraction) -> Result<BigFraction, String> {
        if index > self.dimension {
            return Err(format!("Index {}, dimension {}", index, self.dimension));
        }
        self.numbers[(self.step * index + self.start_pos) as usize] = value.clone();
        return Ok(value);
    }

    pub fn magnitude_sq(&self) -> BigFraction {
        let mut magnitude: BigFraction = BigFraction::get_zero();
        for i in 0..self.dimension {
            magnitude = magnitude.add(self.get(i).unwrap().mul(self.get(i).unwrap()));
        }
        magnitude
    }

    pub fn is_zero(&self) -> bool {
        for i in 0..self.dimension {
            if self.get(i).unwrap().signum() != BigInt::zero() { return false; }
        }
        true
    }

    pub fn add(&mut self, other: BigVector) -> Result<BigVector, String> {
        if self.dimension != other.dimension {
            return Err(format!("The other vector ({}) is not the same dimension ({})", self.dimension, other.dimension));
        }
        for i in 0..self.dimension {
            let res = self.set(i, self.get(i).unwrap().add(other.get(i).unwrap()));
            if res.is_err() {
                return Err(format!("Can not be set for {}", i));
            }
        }
        Ok(other)
    }

    pub fn sub(&mut self, other: BigVector) -> Result<BigVector, String> {
        if self.dimension != other.dimension {
            return Err(format!("The other vector ({}) is not the same dimension ({})", self.dimension, other.dimension));
        }
        for i in 0..self.dimension {
            let res = self.set(i, self.get(i).unwrap().sub(other.get(i).unwrap()));
            if res.is_err() {
                return Err(format!("Can not be set for {}", i));
            }
        }
        Ok(other)
    }

    pub fn mul(&mut self, other: BigVector) -> Result<BigVector, String> {
        if self.dimension != other.dimension {
            return Err(format!("The other vector ({}) is not the same dimension ({})", self.dimension, other.dimension));
        }
        for i in 0..self.dimension {
            let res = self.set(i, self.get(i).unwrap().mul(other.get(i).unwrap()));
            if res.is_err() {
                return Err(format!("Can not be set for {}", i));
            }
        }
        Ok(other)
    }

    pub fn div(&mut self, other: BigVector) -> Result<BigVector, String> {
        if self.dimension != other.dimension {
            return Err(format!("The other vector ({}) is not the same dimension ({})", self.dimension, other.dimension));
        }
        for i in 0..self.dimension {
            let value=self.get(i).unwrap().div(other.get(i).unwrap());
            if value.is_none(){
                return Err(format!("Div impossible for {}", i));
            }
            let res = self.set(i, value.unwrap());
            if res.is_err() {
                return Err(format!("Can not be set for {}", i));
            }
        }
        Ok(other)
    }

    pub fn swap(&mut self, i:usize,j:usize) -> Result<&Self, String> {
        let temp:Result<BigFraction,String>=self.get(i);
        if temp.is_err(){
            return Err(temp.unwrap_err());
        }
        let j_value:Result<BigFraction,String>=self.get(j);
        if j_value.is_err(){
            return Err(j_value.unwrap_err());
        }
        let set_i:Result<BigFraction,String>=self.set(i,j_value.unwrap());
        if set_i.is_err(){
            return Err(set_i.unwrap_err());
        }
        let set_j:Result<BigFraction,String>=self.set(j,temp.unwrap());
        if set_j.is_err(){
            return Err(set_j.unwrap_err());
        }
        Ok(self)
    }


    pub fn dimension(&self) -> usize {
        self.dimension
    }
}
