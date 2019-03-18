pub struct Solution;

impl Solution {
    fn get_list(p: String) -> Vec<String> {
        let mut pc = p.chars();
        let mut p_list: Vec<String> = vec![];
        let mut pp = String::new();
        loop {
            match pc.next() {
                Some(x) => {
                    // println!("x={:#?}", x);
                    if x == '*' {
                        pp.push(x);
                        p_list.push(pp.clone());
                        pp.clear();
                    } else {
                        if pp.len() <= 0 {
                            pp.push(x);
                        } else {
                            p_list.push(pp.clone());
                            pp.clear();
                            pp.push(x);
                        }
                    }
                    // println!("some p_list={:#?}", p_list);
                }
                None => {
                    if 0 < pp.len() {
                        p_list.push(pp);
                    }
                    // println!("none p_list={:#?}", p_list);
                    break;
                }
            }
        }
        return p_list;
    }

    fn match_list(s_list: &Vec<char>, p_list: Vec<String>) -> bool {
        // println!("s_list={:?} p_list={:?}", s_list, p_list);

        let mut s_index = 0;
        let mut p_index = 0;
        let mut p_use = 0;

        loop {
            if s_index < s_list.len() {
                let x = s_list[s_index];
                // println!("s_index={:#?} x={:?}", s_index, x);
                if p_index < p_list.len() {
                    p_use = p_index;
                    let i = p_index;
                    let ch_list: Vec<char> = p_list[i].chars().collect();
                    if 1 < ch_list.len() {
                        let mut s_list2 = s_list.clone();
                        for _i in 0..s_index {
                            s_list2.remove(0);
                        }

                        let mut p_list2 = p_list.clone();
                        for _i in 0..p_index+1 {
                            p_list2.remove(0);
                        }

                        let ret = Solution::match_list(&s_list2, p_list2);
                        if ret {
                            return true;
                        }
                    }
                    // println!("x={:?} i={:?} ch_list={:?}", x, i, ch_list);
                    let ch = ch_list[0];
                    if x == ch || ch == '.' {
                        s_index += 1;
                        if 1 == ch_list.len() {
                            p_index += 1;
                        }
                    } else {
                        if 1 == ch_list.len() {
                            return false;
                        }

                        p_index += 1;
                        // s_index += 1;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        println!("s_index={:#?} s_list.len={:?} p_index={:#?} p_list.len={:#?} p_use={:?}", s_index, s_list.len(), p_index, p_list.len(), p_use);
        if s_list.len() == s_index {
            if p_list.len() == p_use + 1 {
                return true;
            }

            for i in p_index .. p_list.len() {
                let ch_list: Vec<char> = p_list[i].chars().collect();
                if ch_list.len() != 2 {
                    return false;
                }
            }

            return true;
        }

        return false;
    }

    pub fn is_match(s: String, p: String) -> bool {
        println!("s={:?} p={:?}", s, p);

        let p_list = Solution::get_list(p);

        if s.len() <= 0 {
            if p_list.len() <= 0 {
                return true;
            }

            for i in 0..p_list.len() {
                if p_list[i].len() != 2 {
                    return false;
                }
            }
        }

        let s_list: Vec<char> = s.chars().collect();
        return Solution::match_list(&s_list, p_list);
    }

    pub fn test() {
        let mut s = String::from("aa");
        let mut p = String::from("a");
        assert_eq!(false, Solution::is_match(s, p));

        s = String::from("aa");
        p = String::from("aab");
        assert_eq!(false, Solution::is_match(s, p));

        s = String::from("aa");
        p = String::from("a*");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("aa");
        p = String::from(".*");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("aab");
        p = String::from("c*a*b");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("mississippi");
        p = String::from("mis*is*p*.");
        assert_eq!(false, Solution::is_match(s, p));

        s = String::from("mississippi");
        p = String::from("mis*is*ip*.");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("aaa");
        p = String::from("a*a");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("aaaa");
        p = String::from("a*a");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("a");
        p = String::from("a*");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("a");
        p = String::from("aa*");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("a");
        p = String::from("aa*a");
        assert_eq!(false, Solution::is_match(s, p));

        s = String::from("");
        p = String::from(".*");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("");
        p = String::from(".*.*");
        assert_eq!(true, Solution::is_match(s, p));

        s = String::from("");
        p = String::from(".");
        assert_eq!(false, Solution::is_match(s, p));

        s = String::from("");
        p = String::from("");
        assert_eq!(true, Solution::is_match(s, p));
    }
}
