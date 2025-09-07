


pub fn edit_distance(source: &str, target: &str) -> usize {
    edit_distance_recursion(source, source.len(),target ,target.len())
}



fn edit_distance_recursion(source :&str , m :usize , target :&str , n:usize) -> usize{
    if m==0 {
        return n;
    }
    if n==0{
       return  m;
    }

    if source.chars().nth(m-1)==target.chars().nth(n-1) {
        return edit_distance_recursion(source, m-1 , target , n-1);
    }
     (1+ edit_distance_recursion(source, m, target , n-1)).min(1+ edit_distance_recursion(source, m-1 , target, n)).min(
            1+ edit_distance_recursion(source, m-1, target, n-1)
        )
        
}