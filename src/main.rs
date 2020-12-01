use rayon::prelude::*;
use rand::seq::IteratorRandom;
#[derive(Clone)]
struct Node<'a> { // all Nodes should be in the same lifetime
    towards: Vec<&'a i32>, // Node is pointing towards
    from: Vec<&'a i32>,    // Node is is getting pointed from
    value: i32,         // it has a name (value), I have tried to make it a String but rust says "strings do not pocess the copy property"
    pagerank: f32,      // the page rank of the node
}

fn main(){
    //==============================simple graph=====================================
    let mut _a = Node { 
        towards: vec![],
        from: vec![],
        value: 1,
        pagerank: 0.0,
    };
    let mut _b = Node { 
        towards: vec![],
        from: vec![],
        value: 2,
        pagerank: 0.0,
    };
    let mut _c = Node { 
        towards: vec![],
        from: vec![],
        value: 3,
        pagerank: 0.0,
    };
    let mut _d = Node { 
        towards: vec![],
        from: vec![],
        value: 4,
        pagerank: 0.0,
    };
    let mut _e = Node { 
        towards: vec![],
        from: vec![],
        value: 5,
        pagerank: 0.0,
    };
    let mut _f = Node { 
        towards: vec![],
        from: vec![],
        value: 6,
        pagerank: 0.0,
    };
    let mut _g = Node { 
        towards: vec![],
        from: vec![],
        value: 7,
        pagerank: 0.0,
    };

    _a.towards = vec![&_b.value];
    _a.from = vec![&_c.value];

    _b.towards = vec![&_d.value, &_a.value, &_c.value, &_e.value, &_f.value]; // it is very obvious that b is the most important
    _b.from = vec![&_a.value,&_c.value, &_d.value, &_e.value, &_f.value];

    _c.towards = vec![&_a.value,&_b.value,&_d.value];
    _c.from = vec![&_a.value, &_d.value];

    _d.towards = vec![&_c.value];
    _d.from = vec![&_c.value,&_b.value];

    _e.towards = vec![&_f.value];
    _e.from = vec![&_f.value];

    _f.towards = vec![&_e.value];
    _f.from = vec![&_e.value];

    _g.towards = vec![]; // it is very obvious that g is the least important
    _g.from = vec![];    // infact it has no connections. I only added it to see what happens ┌༼◉ل͟◉༽┐

    // puts all the nodes in 1 place
    let mut _simple_graph = vec![_a.clone(),_b.clone(),_c.clone(),_d.clone(),_e.clone(),_f.clone(),_g.clone()];
    let number_of_nodes : Vec<i32> = (0..64).collect();
    let big_graph = create_graph(64, &number_of_nodes);
    //==============================================================================
    println!("{:?}",page_rank(&_simple_graph, 1000));
    println!("{:?}",page_rank(&big_graph,2));
}

fn create_graph<'a>(nodes: i32, numberOfNodes: &Vec<i32>) -> Vec<Node> {
    let mut RandomIterator = rand::thread_rng();
    let mut node_vector: Vec<Node> = vec![];
    let mut numberOfNodesTowards;
    let mut numberOfNodesFrom;
    for i in 0..nodes {
        // get random number of nodes
        numberOfNodesTowards = (1..nodes-1).choose(&mut RandomIterator);
        numberOfNodesFrom = (1..nodes-1).choose(&mut RandomIterator);
        node_vector.push(
            Node {
                towards: (1..nodes-1).choose_multiple(&mut RandomIterator, numberOfNodesTowards.unwrap() as usize).iter().clone().map(|x| numberOfNodes.get(*x as usize).unwrap()).collect(), // choose a random number of nodes, no duplicates
                from: (1..nodes-1).choose_multiple(&mut RandomIterator, numberOfNodesFrom.unwrap() as usize).iter().clone().map(|x| numberOfNodes.get(*x as usize).unwrap()).collect(), // choose a random number of nodes, no duplicates
                value: i,
                pagerank: 0.0,
            }
        );
    }
    return node_vector;
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
    let mut temp_object = _object.clone();
    temp_object.par_iter_mut().for_each(|x| x.pagerank = 1.0/(_object.len() as f32)); // the first iteration sets it all equal
    let mut _prev_iter: Vec<Node>; // previous iteration
    // ========================================DO PAGE RANK====================================================
    for _iteration in 0..n {
        _prev_iter = temp_object.clone(); // set previous iteration
        temp_object.par_iter_mut().for_each(|x| x.pagerank = do_page_rank(x,&_object, &_prev_iter));
    }
    // ========================================================================================================
    let mut _to_compare : Vec<f32> = vec![0.0; temp_object.len()]; // create a vector which will hold the pagerank of each node
    for pageranks in 0.._to_compare.len() {
        _to_compare[pageranks] = temp_object[pageranks].pagerank; // place the page ranks of eachnode in position of _object
    }
    let mut correct_order: Vec<f32> = _to_compare.clone(); // get the correct order of the page ranks
    correct_order.sort_by(|b, a| a.partial_cmp(b).unwrap()); // aka decreasing order, higher the number, the more important
    let mut to_return = vec![0;_object.len()]; // finally create a vector to return the names/values of each node (in correct order)

    let mut for_fun = to_return.clone();

    for order in 0..correct_order.len() {
        /*
        so. (I confused myself reading my own code so Im gonna write a full essay)
        correct_order contains the page ranks in correct order (Vec<f32>)
        _to_compare contains the page ranks (Vec<f32>) in the same order as the objects vector (Vec<Node>).
        so the index of the _to_compare[index] == _object[index].pagerank
        so finding the index of the correct_order in _to_compare, then using it to get the node in _object. 
        we can retrieve the nodes name/value
        */
        to_return[order] = temp_object[get_index(&correct_order[order], &_to_compare) as usize].value;
        for_fun[order] = temp_object[order].value;
    }
    println!("{:?}", for_fun);
    return to_return;
}

// there is probably a better way todo this but alas monkey has got the better of me
fn get_index(number: &f32, vector: &Vec<f32>) -> i32{
    for i in 0..vector.len() {
        if number == &vector[i] {
            return i as i32;
        }
    }
    return -10; // should never be reach but this is here to not make it crash
}

// given a value, find that objects index in object
fn find_the_node(v: i32, _object: &Vec<Node>) -> i32{
    for i in 0.._object.len() {
        if _object[i].value == v {
            return i as i32;
        }
    }
    return -10; // should never be reach but this is here to not make it crash
}

fn do_page_rank(node: &Node, _object: &Vec<Node>, _prev_iter: &Vec<Node>) -> f32{
    let mut to_return: f32 = 0.0; // used to tally up the answer
    let mut index;
    for &node in &node.from { // The node.from is a int vector. Which is the index of the object in the objects vector
        index = find_the_node(*node, _object); // fixes the issue of if you wanted to name the objects something else
        to_return = to_return + (_prev_iter[index as usize].pagerank as f32) / (_object[index as usize].from.len() as f32)
    }
    return to_return;
}