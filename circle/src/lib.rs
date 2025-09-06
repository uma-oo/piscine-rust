use std::f64::consts::PI;


#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center : Point,
	pub radius :f64,
}

impl Circle {
 pub fn new(x:f64 , y: f64, radius: f64) -> Self {
    Self{ center: Point(x,y), radius: radius}
 }

 pub fn diameter(&self) -> f64{
   2.0*self.radius
 }

 pub fn area(&self)-> f64 {
    PI*self.radius.powi(2)
 }

 //ABS(R0 - R1) <= SQRT((x0 - x1)^2 + (y0 - y1)^2) <= (R0 + R1)

 pub fn intersect(&self , another_cirlce: Circle)-> bool {
    let sum_of_rs = ((self.radius+another_cirlce.radius).powi(2).sqrt()).abs();
    let diff_of_rs = ((self.radius-another_cirlce.radius).powi(2).sqrt()).abs();
    let distance_between_centers = self.center.distance(another_cirlce.center);
    distance_between_centers >= diff_of_rs && distance_between_centers <= sum_of_rs

 }

    
}

#[derive(Debug, Clone, Copy)]
pub struct Point (pub f64, pub f64);
impl Point {
    pub fn distance(&self, another_point: Point)  -> f64 {
    ((self.0-another_point.0 ).powi(2)  +(self.1 -another_point.1 ).powi(2)).sqrt()

    }
  
}