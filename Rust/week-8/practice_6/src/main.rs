fn main() {
    let v = vec![1,2,3,4,5,6,7,8];
    let x = vec![5,6,7,8,9,10,11];

    for index in 0..6{
        let sum = v[index] + x[index];
        println!("{:?}" ,sum);
    }
}
