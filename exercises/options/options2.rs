// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
       if let Some(word) = optional_target {
        assert_eq!(word, target);
       }else{
        println!("Not effetive value.");
       }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;
        
        while cursor >=1 {
          // pop type also <Option>.
          if let Some(integer) = optional_integers.pop() {
            let value = match integer { None => 0, Some(i) => i};
            assert_eq!(value, cursor);
          }
          cursor -= 1;
        }
        assert_eq!(cursor, 0);
    }
}
