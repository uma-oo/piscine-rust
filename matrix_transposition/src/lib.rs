

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix( pub( i32,  i32), pub ( i32,  i32));


pub fn transpose(m: Matrix) ->  Matrix{

   let Matrix((x1, y1), (x2,y2)) = m;
   return  Matrix((x1, x2), (y1,y2))

}