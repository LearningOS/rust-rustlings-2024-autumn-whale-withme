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
  
