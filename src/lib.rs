pub mod smart_calc_with_1 {

    /// # Esta função faz uma soma e adiciona 1
    /// 
    /// # exemplo
    /// 
    /// ```rust
    /// use smart_calc::smart_calc_with_1::sum_plus_one;
    /// 
    /// assert_eq!(42, sum_plus_one(41, 0));
    /// assert_eq!(4, sum_plus_one(1, 2));
    /// assert_eq!(1, sum_plus_one(0, 0));
    /// ```
        
    pub fn sum_plus_one(x: u8, y: u8) -> u8 {
        x + y + 1
    }

    /// # Esta função faz uma subtração e subtrai 1
    ///
    /// - se o primeiro parâmetro for menor ou igual ao segundo, retorna 0
    ///
    /// # exemplo
    ///
    /// ```rust
    /// use smart_calc::smart_calc_with_1::sub_less_one;
    ///
    /// assert_eq!(40, sub_less_one(41, 0));
    /// assert_eq!(0, sub_less_one(6, 6));
    /// assert_eq!(0, sub_less_one(5, 50));
    ///```
    pub fn sub_less_one(x: u8, y: u8) -> u8 {
        //if x < (y + 1) {
        if x <= y {
            0
        } else {
            x - y - 1
        }
        
    }
}

#[cfg(test)] 
mod test{

    use super::*;      
    //use tudo(*) o que tiver no escopo superior(super)

    #[test]
    fn test_sum() {
        let result = smart_calc_with_1::sum_plus_one(5, 6);
        let expected = 12;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_failed() {
        let result = smart_calc_with_1::sub_less_one(5, 6);
        let expected = 0;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_sub_failed2() {
        let result = smart_calc_with_1::sub_less_one(6, 6);
        let expected = 0;
        assert_eq!(result, expected);
    }


    #[test]
    fn test_sub() {
        let result = smart_calc_with_1::sub_less_one(6, 1);
        let expected = 4;
        assert_eq!(result, expected);
    }

    
}