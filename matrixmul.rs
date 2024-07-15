fn dot(v1 : &[i32], v2 : &[i32]) -> i32 {
    let mut dot = 0;
    if v1.len() != v2.len(){
        println!("size of vectors differ");
    } else{
        for i in 0..v1.len(){
           dot += v1[i] * v2[i]; 
        }
    }   
    dot
}
fn mxvec(m: &[[i32;3];3],v:&[i32]){
    println!("running");
    for i in 0..m.len() {
        let mut val = 0;
        for k in 0..m[i].len() {
            val += m[i][k] * v[k];
        }
        println!("{} - {}",val,dot(&m[i],&v));
    }
}

fn main(){
    let numbers: [i32; 3] = [1, 2, 5];
    let mat: [[i32;3]; 3] = [[1,2,3],[1,2,9],[1,2,2]];
    println!("{}",numbers.len());
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            println!("{}",mat[i][j]);
        }
    }
    println!("{}",dot(&numbers, &numbers));
    mxvec(&mat,&numbers);
}
