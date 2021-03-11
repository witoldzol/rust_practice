use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        let mut nums :Vec<i32> = Vec::new();
        nums.push(1);
        nums.push(2);
        let nums2 = vec![1,2];
        assert_eq!(nums, nums2);
    }
    
    #[test]
    fn access_vector_vals() {
       let mut vec = vec![1,2,3];
        assert_eq!(vec[0], 1);
        println!("{:?}", vec);
       vec.remove(1);
        println!("{:?}", vec);

        for x in &mut vec {
           *x += 1;
        }
        println!("{:?}", vec);
    }

    #[test]
    fn hash_maps() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        println!("{:?}", map);

        for (key, val) in map {
            // hash-maps do not preserve order !
            println!("{} => {}", key, val);
        }

    }
}
