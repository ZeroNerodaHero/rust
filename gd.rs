fn sim(x:(f64,f64)) -> f64 {
    x.0 * x.0 * x.1 * x.1 + x.0 * x.0 +  x.1 * x.1
}
fn gsim(x:(f64,f64)) -> (f64,f64) {
    (
        2.0* x.0 * x.1 * x.1 + 2.0 * x.0 +  x.1 * x.1,
        2.0*x.0 * x.0 *  x.1 + x.0 * x.0 +  2.0 * x.1
    )
}
fn add(x:(f64,f64),y:(f64,f64)) -> (f64,f64) {
    (x.0 + y.0,x.1+y.1)
}
fn sub(x:(f64,f64),y:(f64,f64)) -> (f64,f64) {
    (x.0 - y.0,x.1-y.1)
}
fn scale(c:f64,x:(f64,f64)) -> (f64,f64) {
    (c * x.0,c * x.1)
}
fn grad(tol:f64) {
    let mut pos = (4.3,3.2);
    let mut last : f64= 99999.0;
    let mut it = 0;
    let gamma = tol * 1000.0;

    loop{
        let cur = sim(pos);
        if (cur - last).abs() < tol {
            break;
        }
        last = cur;
        pos = sub(pos,scale(gamma,gsim(pos)));
        it+=1;
        
        println!("----------\n{}",it);
        println!("grad: {},{}",gsim(pos).0,gsim(pos).1 );
        println!("{},{} -> {}",pos.0,pos.1,last);

        if it == 10{ break;}
    }
    println!("best {},{} -> {}",pos.0,pos.1,last);
}

fn main(){
    grad(0.00001);
}
