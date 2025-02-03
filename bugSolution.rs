fn main() {
    let mut v = vec![1, 2, 3];
    let mut v_clone = v.clone();
    std::thread::spawn(move || {
        println!("Thread 1 sees {:?}", v_clone);
    });
    v[0] = 4;
    println!("Main thread sees {:?}", v);
}