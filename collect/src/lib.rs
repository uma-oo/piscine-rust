


pub fn bubble_sort(arr: &mut [i32]) {
    let mut i = 0;
    while i<arr.len() {
        if  i+1<arr.len() && arr[i]>arr[i+1]  {
            arr.swap(i, i+1);
            i=0;
        } else {
            i+=1;
        }
    }

}