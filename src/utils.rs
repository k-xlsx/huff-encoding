/// Ration a Vec<T> into ration_count sized rations.
/// 
/// Edge cases:
/// * If cannot ration equally -> dumps the remainder into the last ration.
/// * If Vec is to small -> returns just one ration with the whole Vec in it.
pub fn ration_vec<T: Clone>(vec: &Vec<T>, ration_count: usize) -> Vec<Vec<T>>{
    let mut elements_left = vec.len();
    let elements_per_ration = elements_left / ration_count;
    let mut current_element = 0;

    let mut rations: Vec<Vec<T>> = Vec::new();
    if elements_per_ration == 0{
        rations.push(vec[..].to_vec());
    }
    else{
        for i in 0..ration_count{
            if i == ration_count - 1{
                rations.push(vec[current_element..].to_vec());
                break;
            }
        
            rations.push(vec[current_element..current_element + elements_per_ration].to_vec());
            current_element += elements_per_ration;
            elements_left -= elements_per_ration;
        }
    }

    return rations;
}
