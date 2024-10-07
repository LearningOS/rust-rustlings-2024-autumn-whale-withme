## String and &str  
  
### push
  在字符串后追加的只能是**字符串切片或字符字面量**，使用对应的push_str, push两种方法。
  String类型追加字符串切片之后返回的还是String类型。
  ```rust
  pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
  let mut ret:Vec<String> = Vec::new();
    for word in words{
      let capacity_word = capitalize_first(word);
      ret.push(capacity_word);
    }
  ret
}
  ```

### String and iterator  
  
  string.chars()可以转换字符串到迭代器，配合as_str可以收集剩余的字符，
  应该是.chars()实现了IntoIterstor特征

## unsafe关键字  
  
  在方法、特征前声明`unsafe`,表示这个函数包含不安全的代码。调用这个函数的代码必须在unsafe上下文中进行，
  以提醒调用者要小心处理。上下文指的是需要被`unsafe {}`包裹住，使用unsafe后编译器不会对代码进行部分的安全检查，
  例如：直接访问或修改裸指针、调用外部 C 函数、修改内存布局等底层操作等等  
  ```rust
  unsafe fn modify_by_address(mut address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
      // let ptr = address as *mut u32;
      // *ptr = 0xAABBCCDD;
      *(address as *mut u32) = 0xAABBCCDD;
    }
  }

  fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
  }
  ```
  下面的代码涉及到指针和Box，没理解
  ```rust
  unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe {Box::from_raw(ptr)};
    ret.b = Some(String::from("hello"));
    ret
  }
  fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
  }
  ```
## 迭代器iterator  
    
  **迭代器之所以成为迭代器，就是因为实现了 Iterator 特征，要实现该特征，最主要的就是实现其中的 next 方法**
  `into_iter`拿走所有权，`iter_mut`可变借用，其余都是不可变借用  
  - .iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
  - .iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)

  ```rust
  pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // ...
  }
  ```

  注意区分两个特征Iterator 和 IntoIterator：
  1. Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next
  2.  IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器  
    
  使用filter和counter的例子  
  ```rust
  fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize{
    map.values().filter(|&val| val==value).count()    /* filter 过滤的是map的值 */
  }
  ```
   
### collect  
  
  collect可以把迭代器转换为一个集合类型，如 Vec、HashMap 等。collect() 的返回类型取决于它被用于收集成什么类型，
  通常可以通过上下文来推断
  ```rust
  // Complete the function and return a value of the correct type so the test
  // passes.
  // Desired output: Ok([1, 11, 1426, 3])
  fn result_with_list() -> Result<Vec<i32>, DivisionError> {
      let numbers = vec![27, 297, 38502, 81];
      numbers.into_iter().map(|n| divide(n, 27)).collect()
  }

  // Complete the function and return a value of the correct type so the test
  // passes.
  // Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
  fn list_of_results() -> Vec<Result<i32, DivisionError>> {
      let numbers = vec![27, 297, 38502, 81];
      numbers.into_iter().map(|n| divide(n, 27)).collect()
  }
  ```

## 闭包Closure  
  
## 宏编程macro 

## 智能指针  
  
### Box  
  
  Box简单的封装，数据存储在堆上，使用场景  

  1. 特意的将数据分配在堆上
```rust
fn main() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
}
```
  
  2. 
  
## 多线程  
  
  使用 thread::spawn 可以创建线程：
```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
  
  对共享变量进行修改，需要引用arc是一种线程安全的引用计数智能指针，允许多个线程同时拥有对同一数据的所有权，需要配合锁进行修改
```rust
let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));   
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status_thread = status_shared.lock().unwrap();
            status_thread.jobs_completed += 1;
        });
        handles.push(handle);
    }
```
  
  
  
