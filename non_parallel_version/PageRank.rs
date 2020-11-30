#[derive(Clone)]
struct Node<'a> { // all Nodes should be in the same lifetime
    towards: Vec<&'a i32>, // Node is pointing towards
    from: Vec<&'a i32>,    // Node is is getting pointed from
    value: i32,         // it has a name (value)
}

fn main(){
    //==============================simple graph=====================================
    let mut _a = Node { 
        towards: vec![],
        from: vec![],
        value: 0
    };
    let mut _b = Node { 
        towards: vec![],
        from: vec![],
        value: 1
    };
    let mut _c = Node { 
        towards: vec![],
        from: vec![],
        value: 2
    };
    let mut _d = Node { 
        towards: vec![],
        from: vec![],
        value: 3
    };

    _a.towards = vec![&_b.value,&_c.value];
    _a.from = vec![&_c.value];

    _b.towards = vec![&_d.value];
    _b.from = vec![&_a.value,&_c.value];

    _c.towards = vec![&_a.value,&_b.value,&_d.value];
    _c.from = vec![&_a.value, &_d.value];

    _d.towards = vec![&_c.value];
    _d.from = vec![&_c.value,&_b.value];

    // to allow for look up
    let _simple_graph = vec![_a.clone(),_b.clone(),_c.clone(),_d.clone()];

    //==============================================================================
    println!("{:?}",page_rank(&_simple_graph, 1000));
}
/**
 * function takes in a vector of nodes and a number n for iterations
 * returns the values (names) in order of rank
*/
fn page_rank(_object: &Vec<Node>, n: i32) -> Vec<i32>{
    // the equation is
    // for PageRank of A in iteration i+1 = 
    //         sum of { n = nodes pointing to A}
    //         ( PageRank of n in iteration i / number of nodes n points to )
    let mut _to_compare: Vec<f32> = vec![1.0/(_object.len() as f32); _object.len()]; // our current iteration
    let mut _prev_iter: Vec<f32>; // previous iteration
    // ========================================DO PAGE RANK====================================================
    let mut index: i32;
    let mut temp_value: f32;
    for _iteration in 0..n {
        _prev_iter = _to_compare.clone(); // set previous iteration
        index = 0; // used to index the objects in _to_compare (to set the objects in the current index)
        for node in _object {
            temp_value = do_page_rank(&node, _object, &_prev_iter); // calculate using the formulae
            _to_compare[index as usize] = temp_value; // reset the value
            index = index + 1;
        }
    }
    println!("{:?}", _to_compare);
    // ========================================================================================================
    let mut correct_order: Vec<f32> = _to_compare.clone();
    correct_order.sort_by(|b, a| a.partial_cmp(b).unwrap()); // the correct ordering of the nodes
    println!("{:?}", correct_order);
    let mut to_return: Vec<i32> = vec![0; _to_compare.len()];
    for order in 0..correct_order.len() {
        to_return[order] = get_index(&correct_order[order], &_to_compare);
    }
    // compare the nodes and order them here
    return to_return;
}

fn get_index(number: &f32, vector: &Vec<f32>) -> i32{
    for i in 0..vector.len() {
        if number == &vector[i] {
            return i as i32;
        }
    }
    return -10; // should never be run but here to not make it crash
}

fn do_page_rank(node: &Node, _object: &Vec<Node>, _prev_iter: &Vec<f32>) -> f32{
    let mut to_return: f32 = 0.0; // used to tally up the answer
    for &node in &node.from { // The node.from is a int vector. Which is the index of the object in the objects vector
        to_return = to_return + (_prev_iter[*node as usize] as f32) / (_object[*node as usize].from.len() as f32)
    }
    return to_return;
}