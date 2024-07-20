use std::env;

fn sim(x:(f64,f64)) -> f64 {
    x.0 * x.0 * x.1 * x.1 + x.0 * x.0 +  x.1 * x.1
}
fn gsim(x:(f64,f64)) -> (f64,f64) {
    (
        2.0* x.0 * x.1 * x.1 + 2.0 * x.0 ,
        2.0*x.0 * x.0 *  x.1 + 2.0 * x.1
    )
}
fn hess_sim(x:(f64,f64)) -> ((f64,f64),(f64,f64)) {
    (
        ( 2.0 * x.1 * x.1 + 2.0 , 4.0* x.0 * x.1 ),
        ( 2.0 * x.0 * x.0 + 2.0 , 4.0* x.0 * x.1 )
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
fn matrixmul(a:((f64,f64),(f64,f64)),b:(f64,f64)) -> (f64,f64) {
    ( 
        a.0.0 * b.0 + a.0.1 * b.1, 
        a.1.0 * b.0 + a.1.1 * b.1
    )
}
fn grad(tol:f64) {
    let args: Vec<String> = env::args().collect();
    let mut pos = (4.3,4.3);
    let mut last : f64= 99999.0;
    let mut it = 0;
    let alpha: f64 = args[1].parse().unwrap();

    loop{
        println!("----------\n{}",it);
        println!("grad: {},{}",gsim(pos).0,gsim(pos).1 );
        let hh = hess_sim(pos);
        println!("hess: {},{},{},{}",hh.0.0,hh.0.1,hh.1.0,hh.1.1);
        println!("mul: {},{}",matrixmul(hess_sim(pos),gsim(pos)).0,
            matrixmul(hess_sim(pos),gsim(pos)).1);
        println!("{},{} -> {}",pos.0,pos.1,last);

        let cur = sim(pos);
        if (cur - last).abs() < tol {
            break;
        }

        last = cur;
        pos = sub(pos,scale(alpha,matrixmul(hess_sim(pos),gsim(pos))));
        it+=1;
        //if it == 10{ break;}
    }
    println!("best {},{} -> {}",pos.0,pos.1,last);
}

fn main(){
    grad(0.00001);
}
